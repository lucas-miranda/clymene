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

use colored::Colorize;
use tree_decorator::decorator;

use crate::{
    common::Verbosity,
    graphics::Graphic,
    processors::{
        cache::CacheStatus,
        ConfigStatus,
        image::format_handlers::FormatHandler,
        Processor,
        State
    },
    settings::{
        Config,
        DisplayKind,
        ProcessorConfig
    },
    util
};

pub struct ImageProcessor<'a> {
    verbose: bool,
    format_handlers: Vec<Box<(dyn FormatHandler + 'a)>>
}

impl<'a> ImageProcessor<'a> {
    pub fn new() -> Self {
        ImageProcessor {
            verbose: false,
            format_handlers: Vec::new()
        }
    }

    pub fn register_handler<H: 'a + FormatHandler>(&mut self, handler: H) -> &mut Self {
        self.format_handlers.push(Box::new(handler));
        self
    }
}

impl<'a> Processor for ImageProcessor<'a> {
    fn name(&self) -> &str {
        "Image"
    }

    fn retrieve_processor_config<'c>(&self, config: &'c Config) -> &'c dyn ProcessorConfig {
        &config.image
    }

    fn setup(&mut self, config: &mut Config) -> ConfigStatus {
        let input_pathbuf = PathBuf::from(&config.image.input_path);

        infoln!(block, "Verifying image input path");
        traceln!("At {}", input_pathbuf.display().to_string().bold());
        match input_pathbuf.metadata() {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    panic!("Expected a valid directory at input path '{}'.", input_pathbuf.display());
                } else {
                    infoln!(last, "{}", "Done".green());
                }
            },
            Err(e) => {
                if let io::ErrorKind::NotFound = e.kind() {
                    panic!("Directory not found at input path '{}'.", input_pathbuf.display());
                }

                panic!("{}", e);
            }
        }

        let output_path = config.cache.images_path();

        infoln!(block, "Verifying image output path");
        traceln!("At {}", output_path.display().to_string().bold());

        match output_path.metadata() {
            Ok(_metadata) => {
                infoln!(last, "{}", "Done".green());
                config.image.output_path = output_path;
            },
            Err(e) => {
                if let io::ErrorKind::NotFound = e.kind() {
                    traceln!("Directory not found");
                    infoln!("Creating directory '{}'", output_path.display());

                    fs::create_dir(&output_path).unwrap();

                    infoln!(last, "{}", "Done".green());
                } else {
                    panic!("{}", e);
                }
            }
        }

        let mut config_status = ConfigStatus::NotModified;

        infoln!(block, "Preparing format handlers");
        for handler in &self.format_handlers {
            infoln!(block, "{}", handler.name().bold());
            match handler.setup(config) {
                Ok(handler_config_status) => {
                    // update config status 

                    if let ConfigStatus::Modified = handler_config_status {
                        match config_status {
                            ConfigStatus::Modified => (),
                            ConfigStatus::NotModified => config_status = ConfigStatus::Modified
                        }
                    }

                    infoln!(last, "{}", "Ok".green());
                },
                Err(e) => {
                    traceln!("{}: {}", "Error".bold().red(), e);
                    infoln!(last, "{}", "Fail".red());
                }
            }
        }
        infoln!(last, "{}", "Done".green());

        config_status
    }

    fn execute(&self, state: &mut State) {
        let display_kind = if is_trace_enabled!() {
            DisplayKind::Detailed
        } else {
            state.config.image.display
        };

        infoln!(block, "Looking for source files");

        traceln!(entry: decorator::Entry::None, "Source path: {}", state.config.image.input_path.bold());
        traceln!(entry: decorator::Entry::None, "Target path: {}", state.config.image.output_path.display().to_string().bold());

        // sort files by it's extension
        let mut source_files_by_extension: HashMap<OsString, Vec<PathBuf>> = HashMap::new();
        source_files_by_extension.insert(OsString::default(), Vec::new());
        let default_ext = OsString::default();

        let source_path = PathBuf::from(&state.config.image.input_path);
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
        let output = &mut state.graphic_output;

        // progress bar
        let mut file_count = 0;

        for source_files in source_files_by_extension.values() {
            file_count += source_files.len();
        }

        let bar_length = 20;

        if let DisplayKind::Simple = display_kind {
            info!(
                "[{}]  0/{}  0%", 
                " ".repeat(bar_length),
                file_count,
            );
        }

        for format_handler in &self.format_handlers {
            let source_files = format_handler.extensions()
                    .iter()
                    .filter_map(|ext| source_files_by_extension.remove(&OsString::from(ext)))
                    .flatten()
                    .collect::<Vec<PathBuf>>();

            for (file_index, source_file) in source_files.iter().enumerate() {
                if let DisplayKind::Simple = display_kind {
                    // progress bar
                    let completed_percentage = (file_index as f32) / (source_files.len() as f32);
                    let completed_bar_length = (completed_percentage * (bar_length as f32)).round() as usize;

                    print!("\r");
                    info!(
                        "[{}{}]  {}/{}  {:.2}%           ", 
                        "=".repeat(completed_bar_length),
                        {
                            if completed_bar_length > 0 && completed_bar_length < bar_length {
                                let mut s = ">".to_owned();
                                s.push_str(&" ".repeat(bar_length - completed_bar_length - 1));
                                s
                            } else {
                                " ".repeat(bar_length - completed_bar_length)
                            }
                        },
                        file_index,
                        file_count,
                        completed_percentage * 100f32
                    );
                }

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

                if let DisplayKind::Detailed = display_kind {
                    infoln!(block, "{}", location.display().to_string().bold().cyan());
                }

                if !state.force {
                    // verify cache entry
                    match &state.cache {
                        Some(cache) => {
                            match cache.retrieve(&location, &source_metadata) {
                                CacheStatus::Found(cache_entry) => {
                                    if let DisplayKind::Detailed = display_kind {
                                        infoln!("Cache: {}", "Found".green());
                                    }

                                    if let Some(graphic) = cache_entry.retrieve_graphic(source_file, &cache.images_path) {
                                        match display_kind {
                                            DisplayKind::List => infoln!("{} {}", "*".bold().blue(), location.display().to_string().bold().cyan()),
                                            DisplayKind::Detailed => infoln!(last, "{} {}", "*".blue().bold(), "Include".blue()),
                                            _ => ()
                                        }

                                        match graphic {
                                            Graphic::Empty => (),
                                            _ => output.graphics.push(graphic)
                                        }

                                        continue;
                                    } else {
                                        panic!("Something went wrong. Cache was found, but it's graphic can't be retrieved.\nAt location '{}'", location.display())
                                    }
                                },
                                CacheStatus::NotFound => {
                                    if let DisplayKind::Detailed = display_kind {
                                        infoln!("Cache: {}", "Not Found".red());
                                    }
                                },
                                CacheStatus::Outdated => {
                                    if let DisplayKind::Detailed = display_kind {
                                        infoln!("Cache: {}", "Outdated".yellow());
                                    }
                                }
                            }
                        },
                        None => {
                            panic!("Can't access cache. Isn't at valid state.");
                        }
                    }
                }

                // prepare output path
                let output_path = match source_file.strip_prefix(&state.config.image.input_path) {
                    Ok(p) => {
                        state.config
                             .cache
                             .images_path()
                             .join(p.with_extension(""))
                    },
                    Err(e) => panic!("Can't strip prefix '{}' from source path '{}':\n{}", state.config.image.input_path, source_file.display(), e)
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
                            _ => panic!("{}", e)
                        }
                    }
                }

                // process source file
                match format_handler.process(source_file, &output_path, &state.config) {
                    Ok(processed_file) => {
                        match processed_file {
                            Graphic::Empty => {
                                match display_kind {
                                    DisplayKind::List => infoln!("{} {}", "~".yellow().bold(), location.display().to_string().bold().cyan()),
                                    DisplayKind::Detailed => {
                                        traceln!(entry: decorator::Entry::None, "Graphic is empty");
                                        infoln!(last, "{} {}", "~".yellow().bold(), "Ignore".yellow());
                                    },
                                    _ => ()
                                }

                                continue;
                            },
                            _ => output.graphics.push(processed_file)
                        }

                        match display_kind {
                            DisplayKind::List => infoln!("{} {}", "+".green().bold(), location.display().to_string().bold().cyan()),
                            DisplayKind::Detailed => infoln!(last, "{} {}", "+".green().bold(), "Include".green()),
                            _ => ()
                        }
                    },
                    Err(e) => {
                        match display_kind {
                            DisplayKind::List => infoln!("{} {}", "x".red().bold(), location.display().to_string().bold().cyan()),
                            DisplayKind::Detailed => {
                                errorln!(entry: decorator::Entry::None, "{}: {}", "Error".bold().red(), e);
                                infoln!(last, "{} {}", "x".red().bold(), "Error".red());
                            },
                            _ => ()
                        }
                    }
                }
            }
        }

        if let DisplayKind::Simple = display_kind {
            print!("\r");
            infoln!(
                "[{}]  {}/{}  100%           ", 
                "=".repeat(bar_length),
                file_count,
                file_count
            );
        }

        infoln!(last, "{}", "Done".green());
    }
}

impl<'a> Verbosity for ImageProcessor<'a> {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
