use super::Mode;
use crate::{args::ArgsHandler, settings::Config, GlobalArgs};
use clap::{App, Arg, ArgMatches, SubCommand};

pub struct CacheModeArgs {
    pub global: GlobalArgs,
    pub all: bool,
}

pub(super) struct CacheMode;

impl ArgsHandler for CacheMode {
    type ModeArgs = CacheModeArgs;

    fn name() -> &'static str {
        "clear-cache"
    }

    fn subcommand<'a>() -> Option<App<'a, 'a>> {
        Some(
            SubCommand::with_name(Self::name())
                .about("Clear cache directory")
                .arg(
                    Arg::with_name("all")
                        .long("all")
                        .help("Clear all cache entries"),
                ),
        )
    }

    fn handle(matches: &ArgMatches, global_args: GlobalArgs) -> Self::ModeArgs {
        CacheModeArgs {
            global: global_args,
            all: matches.is_present("all"),
        }
    }
}

impl Mode for CacheMode {
    fn run(_config: Config, _args: &Self::ModeArgs) {
        infoln!("Cache operation");
    }
}
