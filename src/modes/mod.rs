pub mod cache;
pub mod generator;

use crate::{
    args::{Args, ArgsHandler},
    settings::Config,
    GlobalArgs,
};

use cache::CacheMode;
use colored::Colorize;
use generator::{GeneratorMode, GeneratorModeArgs};

trait Mode: ArgsHandler {
    fn run(config: Config, args: &Self::ModeArgs);
}

macro_rules! register_modes {
    {$($mode_type:ty),*} => {
        pub fn register_subcommands(args: &mut Args) {
            $(
                if let Some(subcommand) = <$mode_type>::subcommand() {
                    args.register(subcommand);
                }
            )*
        }

        pub fn run(config: Config, args: Args, global_args: GlobalArgs) {
            if global_args.verbose {
                traceln!("With config file at {}", global_args.config_filepath.bold());
            }

            let (name, m) = args.subcommands();

            $(
                if <$mode_type>::name() == name {
                    if let Some(sub_matches) = m {
                        let submode_args = <$mode_type>::handle(sub_matches, global_args);
                        <$mode_type>::run(config, &submode_args);
                        return;
                    }
                }
            )*

            // default mode
            GeneratorMode::run(config, &GeneratorModeArgs::new(global_args));
        }
    }
}

register_modes! {
    CacheMode,
    GeneratorMode
}
