
mod command_processor;
use command_processor::CommandProcessor;

pub mod data;

mod format_handler;
pub use format_handler::AsepriteFormatHandler;

pub enum AsepriteProcessor {
    Command,
    RawFile
}
