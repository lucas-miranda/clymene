use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{
        self,
        DirEntry
    },
    io,
    path::PathBuf
};

use log::{
    info,
    trace
};

use colored::Colorize;

use crate::{
    graphics::Graphic,
    processors::{
        image::{
            self,
            format_handlers::FormatHandler
        },
        ConfigStatus,
        Data,
        Error,
        Processor
    },
    settings::Config,
    util
};

pub struct ImageProcessor<'a> {
    format_handlers: Vec<Box<(dyn FormatHandler + 'a)>>
}

impl<'a> Processor for ImageProcessor<'a> {
    fn setup(&mut self, config: &mut Config) -> Result<ConfigStatus, Error> {
        let input_pathbuf = PathBuf::from(&config.image.input_path);

        match input_pathbuf.metadata() {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    return Err(
                        image::Error::InvalidInputPath(
                            input_pathbuf.display().to_string()
                        )
                        .into()
                    );
                }
            },
            Err(e) => {
                if let io::ErrorKind::NotFound = e.kind() {
                    return Err(
                        image::Error::InvalidInputPath(
                            input_pathbuf.display().to_string()
                        )
                        .into()
                    );
                }

                return Err(image::Error::IO(e).into());
            }
        }

        info!("Verifying image output path...");
        let output_path = config.cache.images_path();

        match output_path.metadata() {
            Ok(_metadata) => {
                info!("Ok!");
                config.image.output_path = output_path;
            },
            Err(e) => {
                if let io::ErrorKind::NotFound = e.kind() {
                    info!("Trying to create '{}'...", output_path.display());

                    fs::create_dir(&output_path)
                       .map_err(|_e| {
                           image::Error::InvalidOutputPath(output_path.display().to_string()).into()
                       })?;
                } else {
                    return Err(image::Error::IO(e).into());
                }
            }
        }

        let mut config_status = ConfigStatus::NotModified;

        info!("Preparing format handlers...");
        for handler in &self.format_handlers {
            match handler.setup(config) {
                Ok(handler_config_status) => {
                    // update config status 

                    if let ConfigStatus::Modified = handler_config_status {
                        match config_status {
                            ConfigStatus::Modified => (),
                            ConfigStatus::NotModified => config_status = ConfigStatus::Modified
                        }
                    }

                    info!("  {}  {}", handler.name(), "Ok".green());
                },
                Err(e) => {
                    info!("  {}  {}", handler.name(), "Fail".red());
                    let image_err: image::Error = e.into();
                    return Err(image_err.into());
                }
            }
        }

        Ok(config_status)
    }

    fn execute(&self, data: &mut Data) -> Result<(), Error> {
        trace!("| Source: {}", data.config.image.input_path);
        trace!("| Target: {}", data.config.image.output_path.display());

        info!("> Looking for source files...");

        // sort files by it's extension
        let mut source_files_by_extension: HashMap<OsString, Vec<PathBuf>> = HashMap::new();
        source_files_by_extension.insert(OsString::default(), Vec::new());
        let default_ext = OsString::default();

        let source_path = PathBuf::from(&data.config.image.input_path);
        util::fs::for_every_file(
            &source_path,
            &mut |entry: &DirEntry| {
                let path_buf = entry.path();

                let source_files = match path_buf.extension() {
                    Some(ext) => {
                        let ext_osstring = ext.to_os_string();

                        match source_files_by_extension.get_mut(&ext_osstring) {
                            Some(files) => {
                                files
                            },
                            None => {
                                source_files_by_extension.insert(ext.to_os_string(), Vec::new());
                                source_files_by_extension.get_mut(&ext_osstring)
                                                         .unwrap()
                            }
                        }
                    },
                    None => {
                        source_files_by_extension.get_mut(&default_ext)
                                                 .unwrap()
                    }
                };

                source_files.push(path_buf.as_path().to_owned());
            }
        ).map_err(|e| image::Error::IO(e).into())?;


        // process every format and collect it's graphic data (as image or animation)
        let output = &mut data.graphic_output;

        for format_handler in &self.format_handlers {
            let source_files = format_handler.extensions()
                    .iter()
                    .filter_map(|ext| source_files_by_extension.remove(&OsString::from(ext)))
                    .flatten()
                    .collect::<Vec<PathBuf>>();

            for source_file in &source_files {
                let location;

                match source_file.strip_prefix(&source_path) {
                    Ok(path) => {
                        location = path
                    },
                    Err(_) => {
                        continue;
                    }
                }

                if let Some(cache_entry) = data.cache.get(location) {
                    if let Some(graphic) = cache_entry.retrieve_graphic() {
                        info!("+ {}", source_file.display().to_string().green());

                        match graphic {
                            Graphic::Empty => (),
                            Graphic::Image(image) => output.images.push(image),
                            Graphic::Animation(animation) => output.animations.push(animation)
                        }

                        continue;
                    }
                }

                let output_path = match source_file.file_stem() {
                    Some(stem) => data.config.cache.images_path().join(stem),
                    None => continue
                };

                match format_handler.process(source_file, &output_path, &data.config) {
                    Ok(processed_file) => {
                        match processed_file {
                            Graphic::Empty => {
                                info!("- {}", location.display().to_string().red());
                                trace!("Ignoring it, empty graphic.");
                                continue;
                            },
                            Graphic::Image(image) => output.images.push(image),
                            Graphic::Animation(animation) => output.animations.push(animation)
                        }

                        info!("+ {}", location.display().to_string().green());
                    },
                    Err(e) => {
                        info!("- {}", location.display().to_string().red());
                        trace!("{}: {}", "Error".bold().red(), e);
                    }
                }
            }
        }

        Ok(())
    }
}

impl<'a> ImageProcessor<'a> {
    pub fn new() -> Self {
        ImageProcessor {
            format_handlers: Vec::new()
        }
    }

    pub fn register_handler<H: 'a + FormatHandler>(&mut self, handler: H) -> &mut Self {
        self.format_handlers.push(Box::new(handler));
        self
    }
}
