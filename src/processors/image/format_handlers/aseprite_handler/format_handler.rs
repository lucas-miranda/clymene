use std::{
    convert::TryInto,
    ffi::OsStr,
    fs::{
        self,
        DirEntry,
        OpenOptions
    },
    io::{
        self,
        BufRead,
        Read,
        Seek,
        SeekFrom
    },
    path::{
        Path,
        PathBuf
    },
    process::Command
};

use log::{
    error,
    trace,
    info,
    warn
};

use crate::{
    graphics::{
        animation::{
            Animation,
            Frame,
            Track
        },
        Graphic,
        Image
    },
    math::{
        Rectangle,
        Size
    },
    processors::{
        image::format_handlers::{
            self,
            aseprite_handler::data::{
                Data,
                FrameData
            },
            Error,
        },
        ConfigStatus
    },
    settings::Config,
    util
};

const ASEPRITE_FILE_MAGIC_NUMBER: [u8; 2] = [ 0xE0, 0xA5 ];
const FRAME_FILE_NAME_EXTENSION: &str = "png";
const FRAME_FILE_NAME_FORMAT: &str = "{frame}.png";
const DATA_FILE_NAME: &str = "data.json";

pub struct FormatHandler {
}

impl format_handlers::FormatHandler for FormatHandler {
    fn name(&self) -> &'static str {
        "Aseprite"
    }

    fn extensions(&self) -> &[&str] {
        &[ "ase", "aseprite" ]
    }

    fn setup(&self, config: &mut Config) -> Result<ConfigStatus, Error> {
        let mut config_status = ConfigStatus::NotModified;

        if let ConfigStatus::Modified = self.verify_aseprite_bin(config) {
            config_status = ConfigStatus::Modified;
        }

        Ok(config_status)
    }

    fn process(&self, source_file_path: &Path, output_dir_path: &Path, config: &Config) -> Result<Graphic, Error> {
        trace!("|| aseprite file => {}", source_file_path.display());

        // check if file is valid
        match source_file_path.metadata() {
            Ok(metadata) => {
                if !metadata.is_file() {
                    return Err(Error::FileExpected(source_file_path.to_path_buf()));
                }

                // check magic number section
                let mut file = OpenOptions::new()
                                           .read(true)
                                           .open(&source_file_path)
                                           .unwrap();

                file.seek(SeekFrom::Start(4)).unwrap(); // seek to magic number

                let mut buffer = [0u8; 2];
                file.read_exact(&mut buffer).unwrap();

                if buffer[..] != ASEPRITE_FILE_MAGIC_NUMBER[..] {
                    // magic number doesn't match
                    return Err(Error::WrongFileType);
                }
            },
            Err(e) => {
                panic!(e)
            }
        }

        // verify output directory
        trace!("|| output path => {}", output_dir_path.display());

        match output_dir_path.metadata() {
            Ok(metadata) => {
                if metadata.is_dir() {
                    Ok(())
                } else {
                    Err(Error::DirectoryExpected)
                }
            },
            Err(e) => panic!(e)
        }?;

        // extract every frame (excluding empty ones)
        let frame_pathbuf = output_dir_path.join(FRAME_FILE_NAME_FORMAT);

        trace!("|| frames path format => {}", frame_pathbuf.display());

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
                OsStr::new("--save-as"), frame_pathbuf.as_os_str()
            ])
            .output()
            .unwrap();

        if !output.status.success() {
            return Err(Error::ExternalProgramFail(output.stderr));
        }

        // generate data
        let data_pathbuf = output_dir_path.join(DATA_FILE_NAME);

        trace!("|| data path => {}", data_pathbuf.display());

        let output = Command::new(&config.image.aseprite.bin_path)
            .args(&[
                // batch, do not start UI
                OsStr::new("-b"), 

                // .ase/.aseprite file path
                source_file_path.as_os_str(), 

                // save .json data as
                OsStr::new("--data"), data_pathbuf.as_os_str(), 

                // json format (hash or array)
                OsStr::new("--format"), OsStr::new("json-array"), 

                // show tags data
                OsStr::new("--list-tags"),

                // trim empty space
                OsStr::new("--trim") 
            ])
            .output()
            .unwrap();

        if !output.status.success() {
            return Err(Error::ExternalProgramFail(output.stderr));
        }

        // process generated images and data

        let aseprite_data = Data::from_file(&data_pathbuf)
                                 .map_err(|e| e.into())?;

        // retrieve source images
        let mut source_images = self.find_source_images(&source_file_path, &output_dir_path, &aseprite_data.frames);

        if source_images.is_empty() {
            return Ok(Graphic::Empty);
        }

        if source_images.len() == 1 && aseprite_data.meta.frame_tags.is_empty() && aseprite_data.meta.slices.is_empty() {
            // single image
            return Ok(source_images.remove(0).image.into());
        }

        let mut animation = Animation::new(source_file_path.to_owned())
                                      .map_err(|e| e.into())?;

        // register source images
        for source_image in source_images.drain(..) {
            animation.insert_source_image(source_image.frame_index, source_image.image);
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

            for index in frame_tag_data.from..(frame_tag_data.to + 1) {
                if index < 0 {
                    error!("Skipping invalid index '{}'.", index);
                    continue;
                }

                let index_u32 = index as u32;

                let duration = {
                    match index.try_into() {
                        Ok(i) => {
                            match aseprite_data.frames.get::<usize>(i) {
                                Some(frame_data) => frame_data.duration,
                                None => 0
                            }
                        },
                        Err(e) => {
                            error!("When trying to convert index type into usize.");
                            panic!(e);
                        }
                    }
                };

                track.push(Frame::new(
                    index_u32,
                    {
                        if duration < 0 {
                            0
                        } else {
                            duration as u32
                        }
                    }
                ));
            }

            animation.push_track(track);
        }

        Ok(animation.into())
    }
}

impl FormatHandler {
    pub fn new() -> Self {
        Self {
        }
    }

    fn verify_aseprite_bin(&self, config: &mut Config) -> ConfigStatus {
        // confirm config.aseprite.bin_path holds a valid aseprite bin path
        let c = &mut config.image.aseprite;

        if !c.bin_path.is_empty() {
            match self.find_aseprite_bin_path(&c.bin_path) {
                Some(pathbuf) => {
                    if pathbuf == PathBuf::from(&c.bin_path) {
                        // doesn't need to do anything else
                        return ConfigStatus::NotModified;
                    }

                    c.bin_path = pathbuf.display().to_string();
                    info!("Aseprite bin found at '{}'.", c.bin_path);
                    return ConfigStatus::Modified;
                },
                None => {
                    warn!("Can't find aseprite bin at '{}'.", c.bin_path);
                }
            };
        } else {
            warn!("Aseprite bin path undefined.");
        }

        // get aseprite bin path

        let mut line_input = String::new();
        let stdin = io::stdin();

        let ase_filepath: PathBuf;

        'bin_search: loop {
            trace!("> Please, enter Aseprite path: ");

            match stdin.lock().read_line(&mut line_input) {
                Ok(_bytes) => {
                    // remove whitespace at the end
                    let len = line_input.trim_end_matches(&['\r', '\n'][..]).len();
                    line_input.truncate(len);

                    if let Some(pathbuf) = self.find_aseprite_bin_path(&line_input) {
                        ase_filepath = pathbuf;
                        break 'bin_search;
                    }

                    error!("> Aseprite not found at entered path");
                },
                Err(e) => {
                    panic!(e);
                }
            };

            line_input.clear();
        }

        info!("|- Aseprite found!");
        c.bin_path = ase_filepath.display().to_string();

        ConfigStatus::Modified
    }

    fn find_aseprite_bin_path(&self, input: &str) -> Option<PathBuf> {
        let pathbuf = PathBuf::from(input);

        let metadata = match pathbuf.metadata() {
            Ok(metadata) => metadata,
            Err(_e) => {
                return None;
            }
        };

        let aseprite_executable_name = if cfg!(target_os = "windows") {
            "aseprite.exe"
        } else {
            "aseprite"
        };

        if metadata.is_file() {
            match pathbuf.file_name() {
                Some(filename) => {
                    let f = filename.to_str().unwrap();
                    info!("aseprite filename: {}", f);

                    if f.to_lowercase().eq(aseprite_executable_name) {
                        Some(pathbuf)
                    } else {
                        None
                    }
                },
                None => None
            }
        } else if metadata.is_dir() {
            match util::fs::find(
                pathbuf, 
                &mut move |e: &DirEntry| {
                    aseprite_executable_name.eq(&e.file_name().to_str().unwrap().to_lowercase())
                }
            ) {
                Ok(entry) => {
                    match entry {
                        Some(found_entry) => Some(found_entry.path()),
                        None => None
                    }
                },
                Err(_) => None
            }
        } else {
            None
        }
    }

    fn find_source_images(&self, source_file_path: &Path, images_folder_path: &Path, frames_data: &[FrameData]) -> Vec<SourceImage> {
        let mut images = Vec::new();

        for dir_entry in fs::read_dir(images_folder_path).unwrap() {
            let entry = dir_entry.unwrap();
            let path = entry.path();

            if let Ok(metadata) = entry.metadata() {
                if !metadata.is_file() {
                    continue;
                }

                // verify extension
                match path.extension() {
                    Some(ext) => {
                        if ext != FRAME_FILE_NAME_EXTENSION {
                            continue;
                        }
                    },
                    None => continue
                };

                // frame index
                let frame_index: u32 = match path.file_stem() {
                    Some(stem) => {
                        match stem.to_str() {
                            Some(stem_str) => {
                                match stem_str.parse() {
                                    Ok(index) => index,
                                    Err(_) => continue
                                }
                            },
                            None => continue
                        }
                    },
                    None => continue
                };

                let dimensions: Size<u32>;
                let source_region: Rectangle<u32>;

                match frames_data.get(frame_index as usize) {
                    Some(frame_data) => {
                        dimensions = Size::with(frame_data.source_size.w, frame_data.source_size.h)
                                          .unwrap_or_else(Size::default);

                        source_region = Rectangle::with(
                            frame_data.sprite_source_size.x,
                            frame_data.sprite_source_size.y,
                            frame_data.sprite_source_size.w,
                            frame_data.sprite_source_size.h
                        ).unwrap_or_else(Rectangle::default);
                    },
                    None => {
                        dimensions = Size::default();
                        source_region = Rectangle::default();
                    }
                }

                if let Ok(image) = Image::new(path, source_file_path.to_owned(), dimensions, source_region) {
                    images.push(SourceImage { image, frame_index });
                }
            }
        }

        images
    }
}

struct SourceImage {
    pub image: Image,
    pub frame_index: u32
}
