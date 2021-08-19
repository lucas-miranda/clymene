
mod command_processor;
use command_processor::CommandProcessor;

pub mod data;

mod format_handler;
pub use format_handler::AsepriteFormatHandler;

mod raw_file_processor;
use raw_file_processor::RawFileProcessor;

#[allow(dead_code)]
pub enum AsepriteProcessor {
    Command,
    RawFile
}
