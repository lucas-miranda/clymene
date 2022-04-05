use crate::GlobalArgs;
use clap::{command, Arg, ArgMatches, Command};

pub trait ArgsHandler {
    type ModeArgs;

    fn name() -> &'static str;
    fn subcommand<'a>() -> Option<Command<'a>>;
    fn handle(matches: &ArgMatches, global_args: GlobalArgs) -> Self::ModeArgs;
}

pub struct Args<'a> {
    subcommands: Vec<Command<'a>>,
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

    pub fn register(&mut self, subcommand: Command<'a>) {
        self.subcommands.push(subcommand);
    }

    pub fn load(&mut self) {
        let command = command!()
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

        self.matches = Some(
            command
                .subcommands(self.subcommands.drain(..))
                .get_matches(),
        );
    }
}
