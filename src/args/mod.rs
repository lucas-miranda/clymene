use crate::settings::DisplayKind;
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

const DEFAULT_FILEPATH: &str = "config.toml";

pub struct Args {
    pub config_filepath: String,
    pub display: Option<DisplayKind>,
    pub verbose: bool,
    pub debug: bool,
    pub force: bool,
}

impl Args {
    pub fn load() -> Self {
        let matches = app_from_crate!()
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .takes_value(true)
                    .value_name("FILE")
                    .help("Uses a custom config toml file"),
            )
            .arg(
                Arg::with_name("display")
                    .short("d")
                    .long("display")
                    .takes_value(true)
                    .possible_values(&["simple", "list", "detailed"])
                    .case_insensitive(true)
                    .help("Specifies file presentation kind"),
            )
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .help("Gives additional info about execution"),
            )
            .arg(
                Arg::with_name("debug")
                    .long("debug")
                    .help("Gives debug info, useful when tracking problems"),
            )
            .arg(
                Arg::with_name("force")
                    .short("f")
                    .long("force")
                    .help("Force generation of everything, effectively ignoring any cached data"),
            )
            .get_matches();

        Self {
            config_filepath: matches
                .value_of("config")
                .unwrap_or(DEFAULT_FILEPATH)
                .to_owned(),
            display: match matches.value_of("display") {
                Some(d) => match d {
                    "simple" => Some(DisplayKind::Simple),
                    "list" => Some(DisplayKind::List),
                    "detailed" => Some(DisplayKind::Detailed),
                    _ => None,
                },
                None => None,
            },
            verbose: matches.is_present("verbose"),
            debug: matches.is_present("debug"),
            force: matches.is_present("force"),
        }
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
