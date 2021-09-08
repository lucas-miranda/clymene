use clap::ArgMatches;

const DEFAULT_FILEPATH: &str = "config.toml";

pub struct GlobalArgs {
    pub config_filepath: String,
    pub verbose: bool,
    pub debug: bool,
    pub force: bool,
}

impl GlobalArgs {
    pub fn handle(matches: &ArgMatches) -> Self {
        Self {
            config_filepath: matches
                .value_of("config")
                .unwrap_or(DEFAULT_FILEPATH)
                .to_owned(),
            verbose: matches.is_present("verbose"),
            debug: matches.is_present("debug"),
            force: matches.is_present("force"),
        }
    }
}

impl Default for GlobalArgs {
    fn default() -> Self {
        Self {
            config_filepath: DEFAULT_FILEPATH.to_owned(),
            verbose: false,
            debug: false,
            force: false,
        }
    }
}
