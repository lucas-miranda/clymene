use simple_logger::SimpleLogger;

mod args;
use args::Args;

mod graphics;
mod math;

mod processors;
use processors::{
    config::ConfigProcessor,
    cache::{
        CacheImporterProcessor,
        CacheExporterProcessor
    },
    data::DataProcessor,
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
                settings::LoadError::FileNotFound(path) => {
                    log::trace!("Config file created at '{}'.", path.display());
                    let c = Config::default();
                    c.save_to_path(&path).unwrap();
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

    ProcessorsPipeline::new()
                       .enqueue(ConfigProcessor::new())             // ensure essential config are working and prepare it to be at valid state
                       .enqueue(CacheImporterProcessor::new())      // import cache entries and prepares them to the next steps
                       .enqueue(image_processor)                    // handle source images to be at expected format
                       .enqueue(PackerProcessor::new())             // retrieve every image and packs into a single atlas
                       .enqueue(CacheExporterProcessor::new())      // exports cache entries into file format again (to be reusable in next usage)
                       .enqueue(DataProcessor::new())               // get every data from previous steps and packs it together into a nicer format
                       .start(&mut config, &args);
}
