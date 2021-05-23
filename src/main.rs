use flexi_logger::{
    Logger,
    LogSpecBuilder
};

mod args;
use args::Args;

mod common;
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
use settings::{
    Config,
    ProcessorConfig
};

mod util;

fn main() {
    let mut builder = LogSpecBuilder::new();
    builder.default(log::LevelFilter::Info);

    let mut logger_reconf_handle = Logger::with(builder.build())
                                          .check_parser_error()
                                          .unwrap()
                                          .format_for_stdout(flexi_logger::colored_default_format)
                                          .format_for_stderr(flexi_logger::colored_default_format)
                                          .start()
                                          .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

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
        builder.default(log::LevelFilter::Trace);
        config.verbose = true;
    } else {
        config.configure_logger(&mut builder);
    }

    logger_reconf_handle.set_new_spec(builder.build());

    //

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
