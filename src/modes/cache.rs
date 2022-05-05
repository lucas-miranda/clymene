use super::Mode;
use crate::{args::ArgsHandler, settings::Config, GlobalArgs};
use clap::{Arg, ArgMatches, Command};
use colored::Colorize;
use std::{
    fs,
    io::{self, Write},
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

    fn subcommand<'a>() -> Option<Command<'a>> {
        Some(
            Command::new(Self::name())
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
            infoln!(
                "At cache path: {}",
                cache_root_path.display().to_string().bold()
            );

            warn!(
                "It'll {} cache path and {} entry entirely, are you sure? [{}/{}] ",
                "wipe".bold().red(),
                "every".bold(),
                "y".green(),
                "N".red(),
            );

            io::stdout().flush().map_err(eyre::Error::from).unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(eyre::Error::from)
                .unwrap();

            if input.trim().is_empty() || input.to_lowercase().trim() == "n" {
                infoln!(last, "Operation cancelled");
                return;
            }

            clear_cache_dir(&cache_root_path);
        } else {
            infoln!(block, "Cleaning cache");
            let cache_root_path = PathBuf::from(&config.cache.path);
            infoln!(
                "At cache path: {}",
                cache_root_path.display().to_string().bold()
            );

            if !cache_root_path.is_dir() {
                errorln!(last, "Cache root directory doesn't exists");
                return;
            }

            infoln!("identifier  {}", config.cache.identifier.bold());
            let entry_path = cache_root_path.join(&config.cache.identifier);

            if !entry_path.is_dir() {
                errorln!(last, "Cache entry's directory doesn't exists");
                return;
            }

            warn!(
                "It'll {} provided cache entry's path entirely, are you sure? [{}/{}] ",
                "wipe".bold().red(),
                "y".green(),
                "N".red(),
            );

            io::stdout().flush().map_err(eyre::Error::from).unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(eyre::Error::from)
                .unwrap();

            if input.trim().is_empty() || input.to_lowercase().trim() == "n" {
                infoln!(last, "Operation cancelled");
                return;
            }

            clear_cache_dir(&entry_path);
        }
    }
}

fn clear_cache_dir(path: &Path) {
    if !path.exists() {
        errorln!(last, "Directory doesn't exists");
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
