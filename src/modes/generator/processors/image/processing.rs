use std::{
    collections::HashMap,
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
    sync::mpsc::{Receiver, RecvError},
};

use colored::Colorize;
use tree_decorator::decorator;

use crate::{
    graphics::Graphic,
    modes::generator::processors::cache::{Cache, CacheStatus},
    settings::DisplayKind,
    util,
};

use super::{
    FormatHandlerEntry, GraphicOutput, Process, ProcessData, ProcessedData, ProcessedInfo,
    ProcessingOptions, ProcessingThread,
};

static PROGRESS_BAR_LENGTH: usize = 20;

/// Process every source file with every format and collect it's graphic data
pub struct Processing {
    source_path: PathBuf,
    cache_images_path: PathBuf,
    source_files_by_extension: HashMap<OsString, Vec<PathBuf>>,
    total_processed_files: usize,
    succeeded_files: u32,
    new_files: u32,
    failed_files: u32,
    failed_cache_retrieve: u32,
}

impl Processing {
    pub fn new(
        source_path: PathBuf,
        cache_images_path: PathBuf,
        source_files_by_extension: HashMap<OsString, Vec<PathBuf>>,
    ) -> Self {
        Self {
            source_path,
            cache_images_path,
            source_files_by_extension,
            total_processed_files: 0,
            succeeded_files: 0,
            new_files: 0,
            failed_files: 0,
            failed_cache_retrieve: 0,
        }
    }

    /*
    pub fn succeeded_files(&self) -> u32 {
        self.succeeded_files
    }
    */

    pub fn new_files(&self) -> u32 {
        self.new_files
    }

    /*
    pub fn failed_files(&self) -> u32 {
        self.failed_files
    }
    */

    pub fn failed_cache_retrieve(&self) -> u32 {
        self.failed_cache_retrieve
    }

    pub(super) fn process<'a, T: Iterator<Item = &'a FormatHandlerEntry>>(
        &mut self,
        threads: &mut Vec<ProcessingThread>,
        receiver: Receiver<ProcessedInfo>,
        format_handlers: T,
        cache: &mut Cache,
        output: &mut GraphicOutput,
        options: ProcessingOptions,
    ) {
        let file_count = self.collect_file_stats();

        if let DisplayKind::Simple = options.display_kind {
            info!("");
            self.display_progress_bar(0, file_count, 0, 0);
        }

        for format_handler in format_handlers {
            let source_files = format_handler
                .extensions()
                .iter()
                .filter_map(|ext| self.source_files_by_extension.remove(&OsString::from(ext)))
                .flatten()
                .collect::<Vec<PathBuf>>();

            for (file_index, source_file) in source_files.iter().enumerate() {
                if let DisplayKind::Simple = options.display_kind {
                    self.display_progress_bar(
                        file_index,
                        file_count,
                        self.succeeded_files,
                        self.failed_files,
                    );
                }

                let location = match source_file.strip_prefix(&self.source_path) {
                    Ok(path) => path.with_extension(""),
                    Err(_) => {
                        self.failed_files += 1;
                        self.total_processed_files += 1;
                        continue;
                    }
                };

                if !options.force {
                    if let Some(graphic) = self.retrieve_from_cache(
                        &location,
                        source_file,
                        cache,
                        &options.display_kind,
                    ) {
                        output.graphics.push(graphic);
                        self.succeeded_files += 1;
                        self.total_processed_files += 1;
                        continue;
                    } else {
                        self.failed_cache_retrieve += 1;
                    }
                }

                let output_path = self.get_or_create_source_file_output_dir(
                    source_file,
                    &self.source_path,
                    &self.cache_images_path,
                );

                // process source file
                // loop until find an available processing thread

                'search: loop {
                    for (i, thread) in threads.iter_mut().enumerate() {
                        if thread.is_waiting {
                            if is_trace_enabled!() {
                                debugln!(
                                    "Sending '{}' to waiting processing thread #{}",
                                    location.display(),
                                    i,
                                );
                            }

                            thread.is_waiting = false;

                            thread
                                .sender
                                .send(Process::Request(ProcessData {
                                    location,
                                    format_handler: format_handler.clone(),
                                    source_filepath: source_file.clone(),
                                    output_path,
                                }))
                                .unwrap();

                            break 'search;
                        }
                    }

                    // wait until receive a message from a processing thread
                    match self.receive(&receiver, output, &options.display_kind) {
                        Ok(result_data) => {
                            self.total_processed_files += 1;

                            if is_trace_enabled!() {
                                debugln!(
                                    "Sending '{}' to processing thread #{}",
                                    location.display(),
                                    result_data.thread_index,
                                );
                            }

                            threads[result_data.thread_index]
                                .sender
                                .send(Process::Request(ProcessData {
                                    location,
                                    format_handler: format_handler.clone(),
                                    source_filepath: source_file.clone(),
                                    output_path,
                                }))
                                .unwrap();

                            break 'search;
                        }
                        Err(e) => panic!("{}", e),
                    }
                }
            }
        }

        // receive lasting processed data

        while self.total_processed_files < file_count {
            match self.receive(&receiver, output, &options.display_kind) {
                Ok(_) => self.total_processed_files += 1,
                Err(e) => panic!("{}", e),
            }
        }

        // stop every processing thread to release resources
        // it will not be used anymore

        for thread in threads.iter_mut() {
            // finalize
            thread.sender.send(Process::Stop).unwrap();
            let join_handle = thread.join_handle.take().unwrap();
            join_handle.join().unwrap();
        }

        if let DisplayKind::Simple = options.display_kind {
            self.display_progress_bar(
                file_count,
                file_count,
                self.succeeded_files,
                self.failed_files,
            );
            println!();
        }
    }

    /*
    fn try_receive(
        &mut self,
        receiver: &Receiver<ProcessedInfo>,
        output: &mut GraphicOutput,
        display_kind: &DisplayKind,
    ) -> Result<ResultData, TryRecvError> {
        match receiver.try_recv() {
            Ok(processed_info) => {
                Ok(self.handle_processed_info(processed_info, output, display_kind))
            }
            Err(e) => match e {
                TryRecvError::Empty => Ok(ResultData::Empty),
                _ => Err(e),
            },
        }
    }
    */

    fn receive(
        &mut self,
        receiver: &Receiver<ProcessedInfo>,
        output: &mut GraphicOutput,
        display_kind: &DisplayKind,
    ) -> Result<ResultData, RecvError> {
        receiver
            .recv()
            .map(|processed_info| self.handle_processed_info(processed_info, output, display_kind))
    }

    fn handle_processed_info(
        &mut self,
        processed_info: ProcessedInfo,
        output: &mut GraphicOutput,
        display_kind: &DisplayKind,
    ) -> ResultData {
        if let DisplayKind::Detailed = display_kind {
            infoln!(
                block,
                "{}",
                processed_info.location.display().to_string().bold().cyan()
            );
        }

        match processed_info.data {
            ProcessedData::Succeeded => {
                match display_kind {
                    DisplayKind::List => infoln!(
                        "{} {}",
                        "~".yellow().bold(),
                        processed_info.location.display().to_string().bold().cyan()
                    ),
                    DisplayKind::Detailed => {
                        traceln!(entry: decorator::Entry::None, "Graphic is empty");
                        infoln!(last, "{} {}", "~".yellow().bold(), "Ignore".yellow());
                    }
                    _ => (),
                }

                self.succeeded_files += 1;
            }
            ProcessedData::New(g) => {
                match display_kind {
                    DisplayKind::List => infoln!(
                        "{} {}",
                        "+".green().bold(),
                        processed_info.location.display().to_string().bold().cyan()
                    ),
                    DisplayKind::Detailed => {
                        infoln!(last, "{} {}", "+".green().bold(), "Include".green())
                    }
                    _ => (),
                }

                self.succeeded_files += 1;
                self.new_files += 1;

                output.graphics.push(g);
            }
            ProcessedData::Failed(ref format_handler_error) => {
                self.failed_files += 1;

                match display_kind {
                    DisplayKind::List => infoln!(
                        "{} {}",
                        "x".red().bold(),
                        processed_info.location.display().to_string().bold().cyan()
                    ),
                    DisplayKind::Detailed => {
                        errorln!(
                            entry: decorator::Entry::None,
                            "{}: {}",
                            "Error".bold().red(),
                            format_handler_error
                        );
                        infoln!(last, "{} {}", "x".red().bold(), "Error".red());
                    }
                    _ => (),
                }
            }
        }

        ResultData {
            thread_index: processed_info.thread_index,
        }
    }

    fn retrieve_from_cache(
        &self,
        location: &Path,
        source_filepath: &Path,
        cache: &mut Cache,
        display_kind: &DisplayKind,
    ) -> Option<Graphic> {
        let source_metadata = source_filepath.metadata().unwrap();

        match cache.retrieve(&location, &source_metadata) {
            CacheStatus::Found(cache_entry) => {
                if let DisplayKind::Detailed = display_kind {
                    infoln!(block, "Cache: {}", "Found".green());
                }

                if let Some(graphic) =
                    cache_entry.retrieve_graphic(source_filepath, &cache.images_path)
                {
                    match display_kind {
                        DisplayKind::List => infoln!(
                            "{} {}",
                            "*".bold().blue(),
                            location.display().to_string().bold().cyan()
                        ),
                        DisplayKind::Detailed => {
                            infoln!(last, "{} {}", "*".blue().bold(), "Include".blue())
                        }
                        _ => (),
                    }

                    if let Graphic::Empty = graphic {
                        return None;
                    }

                    return Some(graphic);
                } else {
                    panic!("Something went wrong. Cache was found, but it's graphic can't be retrieved.\nAt location '{}'", location.display())
                }
            }
            CacheStatus::NotFound => {
                if let DisplayKind::Detailed = display_kind {
                    infoln!("Cache: {}", "Not Found".red());
                }
            }
            CacheStatus::Outdated => {
                if let DisplayKind::Detailed = display_kind {
                    infoln!("Cache: {}", "Outdated".yellow());
                }
            }
        }

        None
    }

    fn get_or_create_source_file_output_dir(
        &self,
        source_filepath: &Path,
        input_path: &Path,
        images_path: &Path,
    ) -> PathBuf {
        let output_path = match source_filepath.strip_prefix(input_path) {
            Ok(p) => images_path.join(p.with_extension("")),
            Err(e) => panic!(
                "Can't strip prefix '{}' from source path '{}':\n{}",
                input_path.display(),
                source_filepath.display(),
                e
            ),
        };

        // ensure output directory, and it's intermediate ones, exists
        match output_path.metadata() {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    panic!(
                        "Output path '{}' already exists and isn't a directory.",
                        output_path.display()
                    );
                }

                // ensure it's empty
                if !util::fs::is_dir_empty(&output_path).unwrap() {
                    fs::remove_dir_all(&output_path).unwrap();
                    util::wait_until(|| !output_path.exists());
                    fs::create_dir(&output_path).unwrap();
                }
            }
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => fs::create_dir_all(&output_path).unwrap(),
                _ => panic!("{}", e),
            },
        }

        output_path
    }

    fn collect_file_stats(&self) -> usize {
        let mut file_count = 0;

        infoln!(block, "File Types");
        for (ext, source_files) in self.source_files_by_extension.iter() {
            infoln!(
                "{}  {} files",
                ext.as_os_str().to_str().unwrap_or("???").bold(),
                source_files.len()
            );

            file_count += source_files.len();
        }

        infoln!(last; entry: decorator::Entry::Double, "Found {} files", file_count);
        file_count
    }

    fn display_progress_bar(
        &self,
        file_index: usize,
        file_count: usize,
        succeeded_files: u32,
        failed_files: u32,
    ) {
        // progress bar
        let completed_percentage = (file_index as f32) / (file_count as f32);
        let completed_bar_length =
            (completed_percentage * (PROGRESS_BAR_LENGTH as f32)).round() as usize;

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

struct ResultData {
    thread_index: usize,
}
