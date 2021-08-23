use std::env;

use crate::settings::DisplayKind;

const DEFAULT_FILEPATH: &str = "config.toml";

pub struct Args {
    pub config_filepath: String,
    pub display: Option<DisplayKind>,
    pub verbose: bool,
    pub debug: bool,
    pub force: bool,
}

impl Args {
    pub fn parse_env() -> Args {
        let mut args = Args::default();
        let mut iter = env::args().collect::<Vec<String>>().into_iter().skip(1);
        let mut i = 0;

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--config" | "-c" => {
                    let value = match iter.next() {
                        Some(next_arg) => next_arg,
                        None => panic!("Arg #{} => Config path value expected.", i),
                    };

                    args.config_filepath = value;
                }
                "--display" | "-d" => {
                    let mut value = match iter.next() {
                        Some(next_arg) => next_arg,
                        None => panic!("Arg #{} => Display kind value expected. Accepted values are: simple, list or detailed.", i),
                    };

                    value.as_mut_str().make_ascii_lowercase();

                    args.display = match value.as_str() {
                        "simple" => Some(DisplayKind::Simple),
                        "list" => Some(DisplayKind::List),
                        "detailed" => Some(DisplayKind::Detailed),
                        _ => None,
                    };
                }
                "--verbose" => {
                    args.verbose = true;
                }
                "--force" | "-f" => {
                    args.force = true;
                }
                "--debug" => {
                    args.debug = true;
                }
                _ => {}
            }

            i += 1;
        }

        /*
        let args: Vec<String> = env::args().collect();
        for i in 1..args.len() {
            match args[i].as_str() {
                "clear-cache" | "-cc" => {
                    args_behaviors::clear_cache(&PathBuf::from(&config.clymene.cache_path))
                },
                "verbose" | "-v" | "--verbose" => {
                    verbose = true
                },
                _ => {
                }
            }
        }
        */

        args
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            config_filepath: DEFAULT_FILEPATH.to_owned(),
            display: None,
            verbose: false,
            debug: false,
            force: false,
        }
    }
}
