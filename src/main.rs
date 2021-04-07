use simple_logger::SimpleLogger;

mod args;
use args::Args;

mod graphics;
mod math;

mod processors;
use processors::{
    config::ConfigProcessor,
    cache::CacheProcessor,
    image::{
        format_handlers::aseprite_handler,
        ImageProcessor,
    },
    packer::PackerProcessor,
    ProcessorsPipeline
};

mod settings;
use settings::Config;

mod util;

fn main() {
    SimpleLogger::new()
                 .init()
                 .unwrap();

    let args = Args::parse_env();

    let mut config = Config::load_from_path(&args.config_filepath)
       .unwrap_or_else(|e| {
            match e {
                settings::LoadError::Deserialize(de_err) => {
                    panic!(
                        "At file '{}'\nError: {:?}\nDetails: {}", 
                        args.config_filepath, 
                        de_err, 
                        de_err.to_string()
                    );
                },
                settings::LoadError::IO(_) => {
                    let c = Config::default();
                    c.save_to_path(&args.config_filepath).unwrap();
                    c
                }
            }
        });

    if args.verbose {
        log::set_max_level(log::LevelFilter::Trace);
    } else {
        log::set_max_level(log::LevelFilter::Info);
    }

    let mut image_processor = ImageProcessor::new();
    image_processor.register_handler(aseprite_handler::FormatHandler::new());

    let result = ProcessorsPipeline::new()
                       .enqueue(ConfigProcessor::new())  // ensure essential config are working and prepare it to be at valid state
                       .enqueue(CacheProcessor::new())   // verifies cache status and prepares it to the next steps
                       .enqueue(image_processor)         // handle source images to be at expected format
                       .enqueue(PackerProcessor::new())  // retrieve every image and packs into a single atlas
                       //.enqueue(DataProcessor::new())    // get every data from previous steps and packs it together into a nicer format
                       .start(&mut config, &args);

    if let Err(e) = result {
        panic!("[{:?}]: {}", e, e);
    }
}
