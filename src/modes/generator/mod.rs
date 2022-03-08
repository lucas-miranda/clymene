pub mod processors;

use crate::{
    args::ArgsHandler,
    settings::{Config, DisplayKind},
    util::Timer,
    GlobalArgs,
};

use clap::{App, Arg, ArgMatches};
use colored::Colorize;

use processors::{
    cache::{CacheExporterProcessor, CacheImporterProcessor},
    config::ConfigProcessor,
    data::DataProcessor,
    image::{format_handlers::aseprite_handler, ImageProcessor},
    output::OutputProcessor,
    packer::{self, PackerProcessor},
    ProcessorsPipeline,
};

use super::Mode;

pub struct GeneratorModeArgs {
    pub global: GlobalArgs,
    pub display: Option<DisplayKind>,
}

impl GeneratorModeArgs {
    pub fn new(global: GlobalArgs) -> Self {
        Self {
            global,
            display: None,
        }
    }
}

pub(super) struct GeneratorMode;

impl ArgsHandler for GeneratorMode {
    type ModeArgs = GeneratorModeArgs;

    fn name() -> &'static str {
        "generate"
    }

    fn subcommand<'a>() -> Option<App<'a>> {
        Some(
            App::new(Self::name())
                .about("Generate an atlas image and data using source files")
                .arg(
                    Arg::new("display")
                        .short('d')
                        .long("display")
                        .takes_value(true)
                        .possible_values(&["simple", "list", "detailed"])
                        .ignore_case(true)
                        .help("Specifies file presentation kind"),
                ),
        )
    }

    fn handle(matches: &ArgMatches, global_args: GlobalArgs) -> Self::ModeArgs {
        GeneratorModeArgs {
            global: global_args,
            display: match matches.value_of("display") {
                Some(d) => match d {
                    "simple" => Some(DisplayKind::Simple),
                    "list" => Some(DisplayKind::List),
                    "detailed" => Some(DisplayKind::Detailed),
                    _ => None,
                },
                None => None,
            },
        }
    }
}

impl Mode for GeneratorMode {
    fn run(config: Config, args: &Self::ModeArgs) {
        let processing_timer = Timer::start();

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
            .enqueue(PackerProcessor::new(packer::RowTightPacker::new()))
            // exports cache entries into file format again (to be reusable in next usage)
            .enqueue(CacheExporterProcessor::new())
            // get every data from previous steps and packs it together into a nicer format
            .enqueue(DataProcessor::new())
            // copies registered output files from cache to user output dir path
            .enqueue(OutputProcessor::default())
            .start(config, args);

        println!();
        infoln!(block, "{}", "Atlas Completed".magenta().bold());
        infoln!(
            last,
            "Generated in {}s",
            processing_timer.end_secs_str().bold()
        );
    }
}
