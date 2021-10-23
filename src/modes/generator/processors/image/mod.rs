use std::{
    path::PathBuf,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, RwLockReadGuard,
    },
    thread::JoinHandle,
};

use crate::{graphics::Graphic, settings::Config};

mod error;
pub mod format_handlers;
mod graphic_output;
mod graphic_source_data;
mod graphic_source_data_set;
mod image_processor;
mod processing;

pub use error::Error;
use format_handlers::FormatHandler;
pub use graphic_output::GraphicOutput;
pub use graphic_source_data::{GraphicSourceData, GraphicSourceDataError};
pub use graphic_source_data_set::GraphicSourceDataSet;
pub use image_processor::ImageProcessor;
use processing::Processing;

type FormatHandlerEntry = Arc<dyn FormatHandler + Sync + Send + 'static>;

struct ProcessingThread {
    pub join_handle: Option<JoinHandle<()>>,
    pub sender: Sender<Process>,
    pub receiver: Receiver<ProcessedInfo>,
    pub is_waiting: bool,
}

struct ProcessedInfo {
    pub location: PathBuf,
    pub data: ProcessedData,
}

enum ProcessedData {
    Succeeded,
    New(Graphic),
    Failed(format_handlers::Error),
}

enum Process {
    Request(ProcessData),
    Stop,
}

struct ProcessData {
    pub location: PathBuf,
    pub format_handler: FormatHandlerEntry,
    pub source_filepath: PathBuf,
    pub output_path: PathBuf,
}

impl ProcessData {
    pub fn process(self, config: RwLockReadGuard<'_, Config>) -> ProcessedInfo {
        match self
            .format_handler
            .process(&self.source_filepath, &self.output_path, &config)
        {
            Ok(processed_file) => {
                if let Graphic::Empty = processed_file {
                    return ProcessedInfo {
                        location: self.location,
                        data: ProcessedData::Succeeded,
                    };
                }

                ProcessedInfo {
                    location: self.location,
                    data: ProcessedData::New(processed_file),
                }
            }
            Err(e) => ProcessedInfo {
                location: self.location,
                data: ProcessedData::Failed(e),
            },
        }
    }
}
