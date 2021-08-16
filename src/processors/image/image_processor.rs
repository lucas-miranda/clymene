use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{
        self,
        DirEntry
    },
    io,
    path::{
        Path,
        PathBuf
    }
};

use colored::Colorize;
use tree_decorator::decorator;

use crate::{
    common::Verbosity,
    graphics::Graphic,
    processors::{
        cache::{
            Cache,
            CacheStatus
        },
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

static PROGRESS_BAR_LENGTH: usize = 20;

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

    fn retrieve_source_files_by_extension(&self, source_path: &Path) -> HashMap<OsString, Vec<PathBuf>> {
        // sort files by it's extension
        let mut source_files_by_extension = self
            .format_handlers
            .iter()
            .map(|h| h.extensions())
            .flatten()
            .map(|ext| (OsString::from(ext), Vec::new()))
            .collect::<HashMap<OsString, Vec<PathBuf>>>();

        // register source files
        util::fs::for_every_file(
            source_path,
            &mut |entry: &DirEntry| {
                let path_buf = entry.path();

                if let Some(ext) = path_buf.extension() {
                    let ext_osstring = ext.to_os_string();
                    if let Some(source_files) = source_files_by_extension.get_mut(&ext_osstring) {
                        source_files.push(path_buf.as_path().to_owned());
                    }
                }
            }
        ).unwrap();

        source_files_by_extension
    }

    fn retrieve_from_cache(&self, location: &Path, source_filepath: &Path, cache: &mut Cache, display_kind: &DisplayKind) -> Option<Graphic> {
        let source_metadata = source_filepath.metadata().unwrap();

        match cache.retrieve(&location, &source_metadata) {
            CacheStatus::Found(cache_entry) => {
                if let DisplayKind::Detailed = display_kind {
                    infoln!("Cache: {}", "Found".green());
                }

                if let Some(graphic) = cache_entry.retrieve_graphic(source_filepath, &cache.images_path) {
                    match display_kind {
                        DisplayKind::List => infoln!("{} {}", "*".bold().blue(), location.display().to_string().bold().cyan()),
                        DisplayKind::Detailed => infoln!(last, "{} {}", "*".blue().bold(), "Include".blue()),
                        _ => ()
                    }

                    if let Graphic::Empty = graphic {
                        return None;
                    }

                    return Some(graphic);
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

        cache.mark_as_outdated();
        None
    }

    fn get_or_create_source_file_output_dir(&self, source_filepath: &Path, input_path: &str, images_path: &Path) -> PathBuf {
        let output_path = match source_filepath.strip_prefix(input_path) {
            Ok(p) => images_path.join(p.with_extension("")),
            Err(e) => panic!("Can't strip prefix '{}' from source path '{}':\n{}", input_path, source_filepath.display(), e)
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

        output_path
    }

    fn display_progress_bar(&self, file_index: usize, file_count: usize, succeeded_files: u32, failed_files: u32) {
        // progress bar
        let completed_percentage = (file_index as f32) / (file_count as f32);
        let completed_bar_length = (completed_percentage * (PROGRESS_BAR_LENGTH as f32)).round() as usize;

        print!("\r");
        info!(
            "{}/{} [{}{}] {:.2}%   {}  {}             ", 
            file_index,
            file_count,
            "=".repeat(completed_bar_length),
            {
                if completed_bar_length > 0 && completed_bar_length < PROGRESS_BAR_LENGTH {
                    let mut s = ">".to_owned();
                    s.push_str(&" ".repeat(PROGRESS_BAR_LENGTH - completed_bar_length - 1));
                    s
                } else {
                    " ".repeat(PROGRESS_BAR_LENGTH - completed_bar_length)
                }
            },
            completed_percentage * 100f32,
            succeeded_files.to_string().blue(),
            failed_files.to_string().red()
        );
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
                config.image.output_path = output_path;
                infoln!(last, "{}", "Done".green());
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

        let source_path = PathBuf::from(&state.config.image.input_path);
        let mut source_files_by_extension = self.retrieve_source_files_by_extension(&source_path);

        // process every format and collect it's graphic data (as image or animation)
        let output = &mut state.graphic_output;

        // progress bar
        let mut file_count = 0;

        infoln!(block, "File Types");
        for (ext, source_files) in source_files_by_extension.iter() {
            infoln!("{}  {} files", ext.as_os_str().to_str().unwrap_or("???").bold(), source_files.len());
            file_count += source_files.len();
        }

        infoln!(entry: decorator::Entry::None, "Found {} files", file_count);
        infoln!(last, "{}", "Done".green());

        if let DisplayKind::Simple = display_kind {
            info!("");
            self.display_progress_bar(0, file_count, 0, 0);
        }

        let cache_images_path = state.config.cache.images_path();
        let mut succeeded_files = 0;
        let mut failed_files = 0;

        for format_handler in &self.format_handlers {
            let source_files = format_handler
                .extensions()
                .iter()
                .filter_map(|ext| source_files_by_extension.remove(&OsString::from(ext)))
                .flatten()
                .collect::<Vec<PathBuf>>();

            for (file_index, source_file) in source_files.iter().enumerate() {
                if let DisplayKind::Simple = display_kind {
                    self.display_progress_bar(file_index, file_count, succeeded_files, failed_files);
                }

                let location;

                match source_file.strip_prefix(&source_path) {
                    Ok(path) => location = path.with_extension(""),
                    Err(_) => {
                        failed_files += 1;
                        continue;
                    }
                }

                if let DisplayKind::Detailed = display_kind {
                    infoln!(block, "{}", location.display().to_string().bold().cyan());
                }

                if !state.force {
                    match &mut state.cache {
                        Some(c) => {
                            if let Some(graphic) = self.retrieve_from_cache(&location, &source_file, c, &display_kind) {
                                output.graphics.push(graphic);
                                succeeded_files += 1;
                                continue;
                            }
                        },
                        None => panic!("Can't access cache. Isn't at valid state.")
                    }
                } else {
                    infoln!("Cache: {}", "Force Skip".bright_red());
                }

                let output_path = self.get_or_create_source_file_output_dir(&source_file, &state.config.image.input_path, &cache_images_path);

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

                                succeeded_files += 1;
                                continue;
                            },
                            _ => output.graphics.push(processed_file)
                        }

                        match display_kind {
                            DisplayKind::List => infoln!("{} {}", "+".green().bold(), location.display().to_string().bold().cyan()),
                            DisplayKind::Detailed => infoln!(last, "{} {}", "+".green().bold(), "Include".green()),
                            _ => ()
                        }

                        succeeded_files += 1;
                    },
                    Err(e) => {
                        failed_files += 1;

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
            self.display_progress_bar(file_count, file_count, succeeded_files, failed_files);
            println!();
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
