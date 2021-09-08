mod command_processor;
pub mod data;
mod format_handler;
mod raw_file_processor;

use command_processor::CommandProcessor;
pub use format_handler::AsepriteFormatHandler;
use raw_file_processor::RawFileProcessor;

#[allow(dead_code)]
pub enum AsepriteProcessor {
    Command,
    RawFile,
}
