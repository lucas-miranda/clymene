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
        cache::CacheStatus,
        ConfigStatus,
        Data,
        image::format_handlers::FormatHandler,
        Processor
    },
    settings::Config,
    util
};

pub struct ImageProcessor<'a> {
    format_handlers: Vec<Box<(dyn FormatHandler + 'a)>>
}

impl<'a> Processor for ImageProcessor<'a> {
    fn name(&self) -> &str {
        "Image"
    }

    fn setup(&mut self, config: &mut Config) -> ConfigStatus {
        let input_pathbuf = PathBuf::from(&config.image.input_path);

        match input_pathbuf.metadata() {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    panic!("Expected a valid directory at input path '{}'.", input_pathbuf.display());
                }
            },
            Err(e) => {
                if let io::ErrorKind::NotFound = e.kind() {
                    panic!("Directory not found at input path '{}'.", input_pathbuf.display());
                }

                panic!(e);
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

                    fs::create_dir(&output_path).unwrap();
                } else {
                    panic!(e);
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
                    trace!("{}: {}", "Error".bold().red(), e);
                }
            }
        }

        config_status
    }

    fn execute(&self, data: &mut Data) {
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
        ).unwrap();

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
                let source_metadata = source_file.metadata().unwrap();

                match source_file.strip_prefix(&source_path) {
                    Ok(path) => {
                        location = path.with_extension("");
                    },
                    Err(_) => {
                        continue;
                    }
                }

                trace!("=> {}", location.display());

                // verify cache entry
                match &data.cache {
                    Some(cache) => {
                        match cache.retrieve(&location, &source_metadata) {
                            CacheStatus::Found(cache_entry) => {
                                trace!("{}", "Cache Found".green());

                                if let Some(graphic) = cache_entry.retrieve_graphic(&source_file) {
                                    info!("+ {}", location.display().to_string().green());

                                    match graphic {
                                        Graphic::Empty => (),
                                        _ => output.graphics.push(graphic)
                                    }

                                    continue;
                                }
                            },
                            CacheStatus::NotFound => {
                                trace!("{}", "Cache Not Found".red());
                            },
                            CacheStatus::Outdated => {
                                trace!("{}", "Cache Outdated".yellow());
                            }
                        }
                    },
                    None => {
                        panic!("Can't access cache. Isn't at valid state.");
                    }
                }

                // prepare output path
                let output_path = match source_file.strip_prefix(&data.config.image.input_path) {
                    Ok(p) => {
                        data.config
                            .cache
                            .images_path()
                            .join(p.with_extension(""))
                    },
                    Err(e) => panic!("Can't strip prefix '{}' from source path '{}':\n{}", data.config.image.input_path, source_file.display(), e)
                };

                // ensure output directory, and it's intermediate ones, exists
                match output_path.metadata() {
                    Ok(metadata) => {
                        if !metadata.is_dir() {
                            panic!("Output path '{}' already exists and isn't a directory.", output_path.display());
                        }

                        // ensure it's empty
                        if !util::fs::is_dir_empty(&output_path).unwrap() {
                            fs::remove_dir_all(&output_path).unwrap();

                            let duration = std::time::Duration::from_millis(10u64);
                            while output_path.exists() {
                                std::thread::sleep(duration);
                            }

                            fs::create_dir(&output_path).unwrap();
                        }
                    },
                    Err(e) => {
                        match e.kind() {
                            io::ErrorKind::NotFound => fs::create_dir_all(&output_path).unwrap(),
                            _ => panic!(e)
                        }
                    }
                }

                // process source file
                match format_handler.process(source_file, &output_path, &data.config) {
                    Ok(processed_file) => {
                        match processed_file {
                            Graphic::Empty => {
                                info!("- {}", location.display().to_string().red());
                                trace!("Ignoring it, empty graphic.");
                                continue;
                            },
                            _ => output.graphics.push(processed_file)
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
