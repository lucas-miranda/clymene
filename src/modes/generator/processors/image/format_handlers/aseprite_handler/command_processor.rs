use std::{
    ffi::OsStr,
    fs::{self, DirEntry},
    io::{self, BufRead},
    path::{Path, PathBuf},
    process::Command,
};

use colored::Colorize;
use tree_decorator::decorator;

use crate::{
    graphics::{
        animation::{Animation, Frame, Track},
        Graphic, Image,
    },
    math::Size,
    modes::generator::processors::{
        image::{
            format_handlers::{Error, FormatProcessor},
            GraphicSourceData, GraphicSourceDataSet,
        },
        ConfigStatus,
    },
    settings::{AsepriteConfig, Config},
    util,
};

use super::data::{Data, FrameData};

const FRAME_FILE_NAME_FORMAT: &str = "{frame}.png";
const DATA_FILE_NAME: &str = "data.json";

#[derive(Default)]
pub struct CommandProcessor {}

impl CommandProcessor {
    fn ensure_bin_exists(&self, c: &mut AsepriteConfig) -> ConfigStatus {
        if !c.bin_path.is_empty() {
            match self.try_find_bin_path(&c.bin_path) {
                Some(pathbuf) => {
                    if pathbuf == PathBuf::from(&c.bin_path) {
                        // registered aseprite bin path at config
                        // already is a valid one
                        //
                        // don't need to do anything else

                        traceln!(
                            entry: decorator::Entry::None,
                            "Found at {}",
                            c.bin_path.bold()
                        );
                        return ConfigStatus::NotModified;
                    }

                    c.bin_path = pathbuf.display().to_string();
                    infoln!("Found at {}", c.bin_path.bold());
                    return ConfigStatus::Modified;
                }
                None => {
                    warnln!("Can't find bin at {}", c.bin_path.bold());
                }
            };
        } else {
            warnln!(entry: decorator::Entry::None, "Bin path not defined");
        }

        c.bin_path = self.locate_bin_path().display().to_string();
        infoln!("{} {}", "Aseprite".green().bold(), "found!".green());

        ConfigStatus::Modified
    }

    fn try_find_bin_path(&self, input: &str) -> Option<PathBuf> {
        let pathbuf = PathBuf::from(input);

        let metadata = match pathbuf.metadata() {
            Ok(metadata) => metadata,
            Err(_e) => {
                return None;
            }
        };

        let aseprite_executable_name = {
            if cfg!(target_os = "windows") {
                "aseprite.exe"
            } else {
                "aseprite"
            }
        };

        if metadata.is_file() {
            match pathbuf.file_name() {
                Some(filename) => {
                    if filename
                        .to_str()
                        .unwrap()
                        .to_lowercase()
                        .eq(aseprite_executable_name)
                    {
                        Some(pathbuf)
                    } else {
                        None
                    }
                }
                None => None,
            }
        } else if metadata.is_dir() {
            match util::fs::find(pathbuf, &mut move |e: &DirEntry| {
                aseprite_executable_name.eq(&e.file_name().to_str().unwrap().to_lowercase())
            }) {
                Ok(entry) => entry.map(|found_entry| found_entry.path()),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    fn locate_bin_path(&self) -> PathBuf {
        let mut line_input = String::new();
        let stdin = io::stdin();

        let ase_filepath: PathBuf;

        'bin_search: loop {
            info!("> Please, enter {} full path: ", "Aseprite".bold().cyan());

            match stdin.lock().read_line(&mut line_input) {
                Ok(_bytes) => {
                    // remove whitespace at the end
                    let len = line_input.trim_end_matches(&['\r', '\n'][..]).len();
                    line_input.truncate(len);

                    if let Some(pathbuf) = self.try_find_bin_path(&line_input) {
                        ase_filepath = pathbuf;
                        break 'bin_search;
                    }

                    errorln!(
                        entry: decorator::Entry::Double,
                        "{} not found at provided path",
                        "Aseprite".bold().cyan()
                    );
                }
                Err(e) => {
                    panic!("{}", e);
                }
            };

            line_input.clear();
        }

        ase_filepath
    }

    fn find_graphic_sources(
        &self,
        images_folder_path: &Path,
        frames_data: &[FrameData],
    ) -> GraphicSourceDataSet {
        let dimensions = {
            let mut d = None;

            for frame_data in frames_data.iter() {
                if let Some(frame_dimensions) =
                    Size::with(frame_data.source_size.w, frame_data.source_size.h)
                {
                    d = Some(frame_dimensions);
                    break;
                }
            }

            d
        };

        let mut data_set = GraphicSourceDataSet {
            sources: Vec::new(),
            dimensions,
        };

        let frames: Vec<_> = frames_data.iter().map(|f| f.into()).collect();

        for entry in fs::read_dir(images_folder_path)
            .unwrap()
            .filter_map(|e| e.ok())
        {
            if let Ok(graphic_source_data) =
                GraphicSourceData::try_from_path(&entry.path(), &frames)
            {
                data_set.sources.push(graphic_source_data);
            }
        }

        data_set
            .sources
            .sort_unstable_by(|a, b| a.frame_index.cmp(&b.frame_index));

        data_set
    }
}

impl FormatProcessor for CommandProcessor {
    fn setup(&self, config: &mut Config) -> Result<ConfigStatus, Error> {
        let mut config_status = ConfigStatus::NotModified;

        if let ConfigStatus::Modified = self.ensure_bin_exists(&mut config.image.aseprite) {
            config_status = ConfigStatus::Modified;
        }

        Ok(config_status)
    }

    fn process(
        &self,
        source_file_path: &Path,
        output_dir_path: &Path,
        config: &Config,
    ) -> Result<Graphic, Error> {
        // extract every frame (excluding empty ones)
        let output = Command::new(&config.image.aseprite.bin_path)
            .args(&[
                // batch, do not start UI
                OsStr::new("-b"),
                // skip empty frames
                OsStr::new("--ignore-empty"),
                // trim empty space
                //OsStr::new("--trim"), // removed since it doesn't work at all when exporting images
                // .ase/.aseprite file path
                source_file_path.as_os_str(),
                // save every frame as
                OsStr::new("--save-as"),
                output_dir_path.join(FRAME_FILE_NAME_FORMAT).as_os_str(),
            ])
            .output()
            .unwrap();

        if !output.status.success() {
            return Err(Error::ExternalProgramFail(output.stderr));
        }

        // generate data
        let data_pathbuf = output_dir_path.join(DATA_FILE_NAME);

        traceln!(
            entry: decorator::Entry::None,
            "  Data filepath: {}",
            data_pathbuf.display().to_string().bold()
        );

        let output = Command::new(&config.image.aseprite.bin_path)
            .args(&[
                // batch, do not start UI
                OsStr::new("-b"),
                // .ase/.aseprite file path
                source_file_path.as_os_str(),
                // save .json data as
                OsStr::new("--data"),
                data_pathbuf.as_os_str(),
                // json format (hash or array)
                OsStr::new("--format"),
                OsStr::new("json-array"),
                // show tags data
                OsStr::new("--list-tags"),
                // trim empty space
                OsStr::new("--trim"),
            ])
            .output()
            .unwrap();

        if !output.status.success() {
            return Err(Error::ExternalProgramFail(output.stderr));
        }

        // process generated images and data
        let aseprite_data = Data::from_file(&data_pathbuf).map_err::<Error, _>(|e| e.into())?;

        // retrieve source images
        let mut graphic_sources_set =
            self.find_graphic_sources(output_dir_path, &aseprite_data.frames);

        if graphic_sources_set.sources.is_empty() {
            return Ok(Graphic::Empty);
        }

        if graphic_sources_set.sources.len() == 1
            && aseprite_data.meta.frame_tags.is_empty()
            && aseprite_data.meta.slices.is_empty()
        {
            // single image
            return Ok(Image::with_graphic_source(
                graphic_sources_set.sources.remove(0).source,
                source_file_path.to_owned(),
            )
            .unwrap()
            .into());
        }

        let mut animation =
            Animation::new(source_file_path.to_owned()).map_err::<Error, _>(|e| e.into())?;

        // register source images
        for (frame_index, source_data) in graphic_sources_set.sources.drain(..).enumerate() {
            match aseprite_data.frames.get(frame_index) {
                Some(frame_data) => {
                    animation.push_frame(Frame::Contents {
                        graphic_source: source_data.source,
                        duration: if frame_data.duration < 0 {
                            0u32
                        } else {
                            frame_data.duration as u32
                        },
                    });
                }
                None => panic!(
                    "Expected frame {} not found. At aseprite data file '{}'.",
                    frame_index,
                    data_pathbuf.display()
                ),
            }
        }

        // register tracks
        for frame_tag_data in &aseprite_data.meta.frame_tags {
            let label = {
                if frame_tag_data.name.is_empty() {
                    None
                } else {
                    Some(frame_tag_data.name.clone())
                }
            };

            let mut track = Track::new(label);

            for index in frame_tag_data.from..=frame_tag_data.to {
                if index < 0 {
                    errorln!("Skipping invalid index '{}'.", index);
                    continue;
                }

                track.frame_indices.push(index as u32);
            }

            track.frame_indices.sort_unstable();
            animation.push_track(track);
        }

        Ok(animation.into())
    }
}
