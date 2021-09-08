use colored::Colorize;

#[macro_use]
mod log;

mod args;
mod common;
mod global_args;
mod graphics;
mod math;
mod modes;
mod settings;
mod util;

use args::Args;
pub use global_args::GlobalArgs;
use settings::Config;

fn main() {
    let mut args = Args::new();
    modes::register_subcommands(&mut args);
    args.load();

    display_header();

    let global_args = GlobalArgs::handle(&args.matches());
    let mut config = Config::load_from_path_or_default(&global_args.config_filepath);

    log::initialize_logger(&mut config, &global_args);
    modes::run(config, args, global_args);
}

fn display_header() {
    println!(" ┌───────────┐");
    println!(" │  {}  │", env!("CARGO_PKG_NAME").bold().magenta());
    println!(" │   v{}  │", env!("CARGO_PKG_VERSION").bold());
    println!(" └───────────┘");
    println!();
}
