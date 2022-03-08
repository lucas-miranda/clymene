use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
    sync::{mpsc::channel, Arc},
    thread,
};

use colored::Colorize;
use tree_decorator::decorator;

use crate::{
    common::Verbosity,
    modes::generator::processors::{ConfigStatus, Processor, State},
    settings::{Config, DisplayKind, ProcessorConfig},
    util::{self, Timer},
};

use super::{
    format_handlers::FormatHandler, FormatHandlerEntry, Process, Processing, ProcessingOptions,
    ProcessingThread,
};

pub struct ImageProcessor {
    verbose: bool,
    format_handlers: Vec<FormatHandlerEntry>,
    threads: Vec<ProcessingThread>,
}

impl ImageProcessor {
    pub fn new() -> Self {
        ImageProcessor {
            verbose: false,
            format_handlers: Vec::new(),
            threads: Vec::new(),
        }
    }

    pub fn register_handler<H: FormatHandler + Sync + Send + 'static>(
        &mut self,
        handler: H,
    ) -> &mut Self {
        self.format_handlers.push(Arc::new(handler));
        self
    }

    fn retrieve_source_files_by_extension(
        &self,
        source_path: &Path,
    ) -> HashMap<OsString, Vec<PathBuf>> {
        // sort files by it's extension
        let mut source_files_by_extension = self
            .format_handlers
            .iter()
            .flat_map(|h| h.extensions())
            .map(|ext| (OsString::from(ext), Vec::new()))
            .collect::<HashMap<OsString, Vec<PathBuf>>>();

        // register source files
        util::fs::for_every_file(source_path, &mut |entry: &DirEntry| {
            let path_buf = entry.path();

            if let Some(ext) = path_buf.extension() {
                let ext_osstring = ext.to_os_string();
                if let Some(source_files) = source_files_by_extension.get_mut(&ext_osstring) {
                    source_files.push(path_buf.as_path().to_owned());
                }
            }
        })
        .unwrap();

        source_files_by_extension
    }
}

impl Processor for ImageProcessor {
    fn name(&self) -> &str {
        "Image"
    }

    fn retrieve_processor_config<'c>(&self, config: &'c Config) -> Option<&'c dyn ProcessorConfig> {
        Some(&config.image)
    }

    fn setup(&mut self, state: &mut State) -> ConfigStatus {
        {
            let mut c = state.config.try_write().expect("Can't acquire write lock");

            // verify paths

            let input_pathbuf = PathBuf::from(&c.image.input_path);

            infoln!(block, "Verifying image input path");
            traceln!("At {}", input_pathbuf.display().to_string().bold());
            match input_pathbuf.metadata() {
                Ok(metadata) => {
                    if !metadata.is_dir() {
                        panic!(
                            "Expected a valid directory at input path '{}'.",
                            input_pathbuf.display()
                        );
                    } else {
                        infoln!(last, "{}", "Ok".green());
                    }
                }
                Err(e) => {
                    if let io::ErrorKind::NotFound = e.kind() {
                        panic!(
                            "Directory not found at input path '{}'.",
                            input_pathbuf.display()
                        );
                    }

                    panic!("{}", e);
                }
            }

            let output_path = c.cache.images_path();

            infoln!(block, "Verifying image output path");
            traceln!("At {}", output_path.display().to_string().bold());

            match output_path.metadata() {
                Ok(_metadata) => {
                    c.image.output_path = output_path;
                    infoln!(last, "{}", "Ok".green());
                }
                Err(e) => {
                    if let io::ErrorKind::NotFound = e.kind() {
                        traceln!("Directory not found");
                        infoln!("Creating directory '{}'", output_path.display());

                        fs::create_dir(&output_path).unwrap();

                        infoln!(last, "{}", "Ok".green());
                    } else {
                        panic!("{}", e);
                    }
                }
            }
        }

        let mut config_status = ConfigStatus::NotModified;

        infoln!(block, "Preparing format handlers");
        let prepare_timer = Timer::start();

        {
            let mut c = state.config.try_write().expect("Can't acquire write lock");
            for handler in &self.format_handlers {
                infoln!(block, "{}", handler.name().bold());
                match handler.setup(&mut c) {
                    Ok(handler_config_status) => {
                        // update config status

                        if let ConfigStatus::Modified = handler_config_status {
                            match config_status {
                                ConfigStatus::Modified => (),
                                ConfigStatus::NotModified => config_status = ConfigStatus::Modified,
                            }
                        }

                        infoln!(last, "{}", "Ok".green());
                    }
                    Err(e) => {
                        traceln!("{}: {}", "Error".bold().red(), e);
                        infoln!(last, "{}", "Fail".red());
                    }
                }
            }
        }

        doneln_with_timer!(prepare_timer);

        config_status
    }

    fn execute(&mut self, state: &mut State) {
        let c = state.config.try_read().expect("Can't acquire read lock");

        let display_kind = if is_trace_enabled!() {
            DisplayKind::Detailed
        } else {
            c.image.display
        };

        infoln!(block, "Looking for source files");

        traceln!(
            entry: decorator::Entry::None,
            "Source path: {}",
            c.image.input_path.bold()
        );
        traceln!(
            entry: decorator::Entry::None,
            "Target path: {}",
            c.image.output_path.display().to_string().bold()
        );

        let source_path = PathBuf::from(&c.image.input_path);
        let source_files_by_extension = self.retrieve_source_files_by_extension(&source_path);
        let mut processing = Processing::new(
            source_path,
            c.cache.images_path(),
            source_files_by_extension,
        );

        // verify cache status
        let force = state.args().global.force;
        if force {
            match display_kind {
                DisplayKind::Simple => (),
                _ => infoln!("{}", "Force Update".bright_yellow()),
            }
        } else if let Some(c) = &state.cache {
            // check if should rescan source directory
            if c.is_updated() && !state.graphic_output.is_requested() {
                let current_cache_metadata = state.create_cache_metadata();

                // check cached and current source directory's modtime
                if c.meta.generation_metadata().image.source_directory_modtime
                    == current_cache_metadata
                        .generation_metadata()
                        .image
                        .source_directory_modtime
                {
                    infoln!(last, "{}", "Already Updated".green());
                    return;
                }
            }

            infoln!("{}", "Needs Update".blue());
        }

        // prepare processing threads

        let thread_num = if c.image.jobs > 0 {
            c.image.jobs as usize
        } else {
            num_cpus::get()
        };

        traceln!("Preparing {} processing threads", thread_num);
        let (processing_sender, processor_receiver) = channel();

        for thread_index in 0..thread_num {
            let (processor_sender, processing_receiver) = channel();
            let sender = processing_sender.clone();
            let c_lock = Arc::clone(&state.config);

            let join_handle = thread::spawn(move || {
                while let Process::Request(data) = processing_receiver.recv().unwrap() {
                    let c = c_lock
                        .try_read()
                        .expect("Can't retrieve a read lock at child thread");

                    sender.send(data.process(thread_index, c)).unwrap();
                }
            });

            self.threads.push(ProcessingThread {
                join_handle: Some(join_handle),
                sender: processor_sender,
                is_waiting: true,
            });
        }

        // execute processing
        let source_files_handling_timer = Timer::start();

        processing.process(
            &mut self.threads,
            processor_receiver,
            self.format_handlers.iter(),
            state.cache.as_mut().unwrap(),
            &mut state.graphic_output,
            ProcessingOptions {
                display_kind,
                force,
            },
        );

        if processing.new_files() > 0 || processing.failed_cache_retrieve() > 0 {
            // mark cache as outdated

            match &mut state.cache {
                Some(c) => c.mark_as_outdated(),
                None => panic!("Can't access cache. Isn't at valid state."),
            }
        }

        doneln_with_timer!(source_files_handling_timer);
    }
}

impl Verbosity for ImageProcessor {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
