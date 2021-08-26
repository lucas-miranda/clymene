use colored::Colorize;
use tree_decorator::{DecoratorBuilder, StandardDecorator};

#[macro_use]
mod log;

mod args;
mod common;
mod graphics;
mod math;
mod processors;
mod settings;
mod util;

use args::Args;
use processors::{
    cache::{CacheExporterProcessor, CacheImporterProcessor},
    config::ConfigProcessor,
    data::DataProcessor,
    image::{format_handlers::aseprite_handler, ImageProcessor},
    packer::PackerProcessor,
    ProcessorsPipeline,
};

use settings::{Config, ProcessorConfig};

static mut LOGGER: Option<log::Logger> = None;

pub fn logger<'a>() -> &'a Option<log::Logger> {
    unsafe { &LOGGER }
}

//

fn main() {
    display_header();

    // TODO  parse_env should provides an error, when parsing failed
    let args = Args::parse_env();
    let mut logger = log::Logger::default();

    if args.debug {
        logger.debug(true);
    }

    if args.verbose {
        logger.verbose(true);
        println!("With config file at {}", args.config_filepath.bold());
    }

    let mut config = load_or_create_config(&args);

    // args display option has higher priority
    if let Some(display) = args.display {
        config.image.display = display;
    }

    // configure logger
    configure_logger(&mut config, logger);

    // tree decorator
    DecoratorBuilder::with(StandardDecorator::new(2)).build();

    //

    let mut image_processor = ImageProcessor::new();
    image_processor.register_handler(aseprite_handler::AsepriteFormatHandler::new(
        aseprite_handler::AsepriteProcessor::RawFile,
    ));

    ProcessorsPipeline::new()
        // ensure essential config are working and prepare it to be at valid state
        .enqueue(ConfigProcessor::new())
        // import cache entries and prepares them to the next steps
        .enqueue(CacheImporterProcessor::new())
        // handle source images to be at expected format
        .enqueue(image_processor)
        // retrieve every image and packs into a single atlas
        .enqueue(PackerProcessor::new())
        // exports cache entries into file format again (to be reusable in next usage)
        .enqueue(CacheExporterProcessor::new())
        // get every data from previous steps and packs it together into a nicer format
        .enqueue(DataProcessor::new())
        .start(&mut config, &args);
}

fn display_header() {
    println!(" ┌───────────┐");
    println!(" │  {}  │", env!("CARGO_PKG_NAME").bold().magenta());
    println!(" │   v{}  │", env!("CARGO_PKG_VERSION").bold());
    println!(" └───────────┘");
}

fn load_or_create_config(args: &Args) -> Config {
    Config::load_from_path(&args.config_filepath).unwrap_or_else(|e| match e {
        settings::LoadError::Deserialize(de_err) => {
            panic!(
                "At file '{}'\nError: {:?}\nDetails: {}",
                args.config_filepath,
                de_err,
                de_err.to_string()
            );
        }
        settings::LoadError::FileNotFound(path) => {
            println!("Config file created at '{}'.", path.display());
            let c = Config::default();
            c.save_to_path(&path).unwrap();
            c
        }
    })
}

fn configure_logger(config: &mut Config, mut logger: log::Logger) {
    let logger_status = settings::ConfigLoggerStatus {
        verbose: logger.is_verbose(),
    };

    config.configure_logger(&mut logger, &logger_status);
    if logger.is_verbose() {
        config.image.display = settings::DisplayKind::Detailed;
    }

    unsafe {
        LOGGER = Some(logger);
    }
}
