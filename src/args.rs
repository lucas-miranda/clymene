use crate::GlobalArgs;
use clap::{app_from_crate, App, Arg, ArgMatches};

pub trait ArgsHandler {
    type ModeArgs;

    fn name() -> &'static str;
    fn subcommand<'a>() -> Option<App<'a>>;
    fn handle(matches: &ArgMatches, global_args: GlobalArgs) -> Self::ModeArgs;
}

pub struct Args<'a> {
    subcommands: Vec<App<'a>>,
    matches: Option<ArgMatches>,
}

impl<'a> Args<'a> {
    pub fn new() -> Self {
        Self {
            subcommands: Vec::new(),
            matches: None,
        }
    }

    pub fn matches(&self) -> &ArgMatches {
        self.matches.as_ref().unwrap()
    }

    pub fn subcommand(&self) -> Option<(&str, &ArgMatches)> {
        self.matches.as_ref().unwrap().subcommand()
    }

    pub fn register(&mut self, subcommand: App<'a>) {
        self.subcommands.push(subcommand);
    }

    pub fn load(&mut self) {
        let app = app_from_crate!()
            .arg(
                Arg::new("config")
                    .short('c')
                    .long("config")
                    .takes_value(true)
                    .value_name("FILE")
                    .help("Uses a custom config toml file"),
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("Gives additional info about execution"),
            )
            .arg(
                Arg::new("debug")
                    .long("debug")
                    .help("Gives debug info, useful when tracking problems"),
            )
            .arg(
                Arg::new("force")
                    .short('f')
                    .long("force")
                    .help("Force every action"),
            );

        self.matches = Some(app.subcommands(self.subcommands.drain(..)).get_matches());
    }
}
