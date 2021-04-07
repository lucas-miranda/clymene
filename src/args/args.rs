use std::{
    env
};

const DEFAULT_FILEPATH: &str = "config.toml";

pub struct Args {
    pub config_filepath: String,
    pub verbose: bool
}

impl Args {
    pub fn parse_env() -> Args {
        let mut args = Args {
            config_filepath: DEFAULT_FILEPATH.to_owned(),
            verbose: false
        };

        let mut iter = env::args()
                           .collect::<Vec<String>>()
                           .into_iter()
                           .skip(1);

        let mut i = 0;

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--config" | "-c" => {
                    let value = match iter.next() {
                        Some(next_arg) => next_arg,
                        None => panic!("Arg #{} => '--config' value expected.", i)
                    };

                    args.config_filepath = value;
                },
                "--verbose" => {
                    args.verbose = true;
                },
                "--force" | "-f" => {
                },
                _ => {
                }
            }

            i += 1;
        }

        /*
        let args: Vec<String> = env::args().collect();
        for i in 1..args.len() {
            match args[i].as_str() {
                "clear-cache" | "-cc" => {
                    args_behaviors::clear_cache(&PathBuf::from(&config.raven.cache_path))
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
