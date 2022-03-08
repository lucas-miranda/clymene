use super::Mode;
use crate::{args::ArgsHandler, settings::Config, GlobalArgs};
use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use std::{
    fs,
    path::{Path, PathBuf},
};

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

    fn subcommand<'a>() -> Option<App<'a>> {
        Some(
            App::new(Self::name())
                .about("Clear cache directory")
                .arg(Arg::new("all").long("all").help("Clear all cache entries")),
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
    fn run(config: Config, args: &Self::ModeArgs) {
        if args.all {
            infoln!(block, "Cleaning all cache");
            let cache_root_path = PathBuf::from(&config.cache.path);
            traceln!(
                "At cache path: {}",
                cache_root_path.display().to_string().bold()
            );

            clear_cache_dir(&cache_root_path);
        } else {
            infoln!(block, "Cleaning cache");
            let cache_root_path = PathBuf::from(&config.cache.path);
            traceln!(
                "At cache path: {}",
                cache_root_path.display().to_string().bold()
            );

            if !cache_root_path.is_dir() {
                infoln!("Cache root directory doesn't exists");
                doneln!();
                return;
            }

            traceln!("identifier  {}", config.cache.identifier.bold());
            clear_cache_dir(&cache_root_path.join(&config.cache.identifier));
        }
    }
}

fn clear_cache_dir(path: &Path) {
    if !path.exists() {
        traceln!("Directory doesn't exists");
        infoln!(last, "{}", "Already Clear".blue());
        return;
    }

    if path.is_dir() {
        remove_dir_all(path);
    } else {
        panic!(
            "Can't clear cache entry at path '{}', it must be a directory.",
            path.display()
        );
    }
}

fn remove_dir_all(path: &Path) {
    match fs::remove_dir_all(&path) {
        Ok(_) => infoln!(last, "{}", "Clear".green()),
        Err(err) => {
            errorln!("{}", err);
            infoln!(last, "{}", "Fail".red());
        }
    }
}
