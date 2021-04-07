use std::{
    path::{ PathBuf },
    env,
    fs
};

pub mod util;

mod settings;
use settings::{ Config };

mod processors;
use processors::{
    Processor,
    AsepriteProcessor,
    CacheProcessor,
    CrunchProcessor,
    DataProcessor
};

mod args_behaviors;

fn main() {
    let config_filename = "config.toml";

    // loading config settings
    let config = match Config::load(&config_filename) {
        Ok(c) => c,
        Err(e) => {
            println!("An error occured when reading to '{}' file.", config_filename);
            println!("Error: {:?}", e.kind());
            println!("Details: {}", e.to_string());
            return;
        }
    };

    // info
    let cache_path = config.raven.cache_instance_path();
    println!("cache fullpath: {}", cache_path.display());

    let mut atlas_cache_path = PathBuf::from(&cache_path);
    atlas_cache_path.push("atlas/");

    let mut images_cache_path = PathBuf::from(&cache_path);
    images_cache_path.push("images/");

    // program options
    let mut verbose = config.raven.verbose;

    // actions before processors take action
    {
        // paths
        let output_path = PathBuf::from(&config.raven.output_path);
        let aseprite_input_path = PathBuf::from(&config.aseprite.input_path);

        // check for special behaviors
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

        // ensure folders exists before processors starts
        let paths = [
            &output_path,
            &cache_path,
            &atlas_cache_path,
            &images_cache_path,
            &aseprite_input_path
        ];

        for path in paths.iter() {
            if !path.exists() {
                println!("> Create '{}' folder", path.display());
                fs::create_dir_all(path.as_path()).unwrap();
            }
        }
    }

    // setup processors
    let mut processors_steps: Vec<&dyn Processor> = Vec::new();

    let cache_processor = CacheProcessor::new(&cache_path);
    let aseprite_processor = AsepriteProcessor::new(&images_cache_path, &config.aseprite);
    let crunch_processor = CrunchProcessor::new(&images_cache_path, &atlas_cache_path, &config.crunch);
    let data_processor = DataProcessor::new(&config);

    processors_steps.push(&cache_processor);
    processors_steps.push(&aseprite_processor);
    processors_steps.push(&crunch_processor);
    processors_steps.push(&data_processor);

    // running processors one by one
    let mut phase_number = 1;
    let processors_count = processors_steps.len();
    for proccessor_entry in &processors_steps {
        println!("=> Phase {}/{}", phase_number, processors_count);

        match (&proccessor_entry).execute(&verbose) {
            Ok(()) => (),
            Err(e) => {
                println!("! Phase Error: {}", e);
                return (); // end program here
            }
        };

        phase_number += 1;
    }
}
