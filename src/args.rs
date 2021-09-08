use crate::GlobalArgs;
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, Arg,
    ArgMatches,
};

pub trait ArgsHandler {
    type ModeArgs;

    fn name() -> &'static str;
    fn subcommand<'a>() -> Option<App<'a, 'a>>;
    fn handle(matches: &ArgMatches, global_args: GlobalArgs) -> Self::ModeArgs;
}

pub struct Args<'a> {
    subcommands: Vec<App<'a, 'a>>,
    matches: Option<ArgMatches<'a>>,
}

impl<'a> Args<'a> {
    pub fn new() -> Self {
        Self {
            subcommands: Vec::new(),
            matches: None,
        }
    }

    pub fn matches(&self) -> &ArgMatches<'a> {
        self.matches.as_ref().unwrap()
    }

    pub fn subcommands(&self) -> (&str, Option<&ArgMatches<'a>>) {
        self.matches.as_ref().unwrap().subcommand()
    }

    pub fn register(&mut self, subcommand: App<'a, 'a>) {
        self.subcommands.push(subcommand);
    }

    pub fn load(&mut self) {
        let app = app_from_crate!()
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .takes_value(true)
                    .value_name("FILE")
                    .help("Uses a custom config toml file"),
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
                    .help("Force every action"),
            );

        self.matches = Some(app.subcommands(self.subcommands.drain(..)).get_matches());
    }
}
