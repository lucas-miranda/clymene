use std::vec::Vec;

use crate::{modes::generator::GeneratorModeArgs, settings::Config};

use super::{ConfigStatus, Processor, State};

pub struct ProcessorsPipeline<'a> {
    processors: Vec<Box<dyn Processor + 'a>>,
}

impl<'a> ProcessorsPipeline<'a> {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub fn start(&mut self, config: Config, args: &GeneratorModeArgs) {
        let mut config_status = ConfigStatus::NotModified;
        let mut state = State::new(config, args);

        for processor in self.processors.iter_mut() {
            {
                let config = state.config.try_read().expect("Can't retrieve a read lock");

                if let Some(c) = processor.retrieve_processor_config(&config) {
                    if c.is_verbose() {
                        processor.verbose(true);
                    }
                }
            }

            match &processor.setup(&mut state) {
                ConfigStatus::NotModified => (),
                ConfigStatus::Modified => {
                    // save processor config status for later use
                    // if it's modified
                    match &config_status {
                        ConfigStatus::NotModified => config_status = ConfigStatus::Modified,
                        ConfigStatus::Modified => (),
                    }
                }
            }
        }

        if let ConfigStatus::Modified = config_status {
            // config was modified, we need to save it to keep updated
            let config = state.config.try_read().expect("Can't retrieve a read lock");

            config.save_to_path(&args.global.config_filepath).unwrap();
        }

        for processor in self.processors.iter_mut() {
            processor.execute(&mut state);
            println!();
        }
    }

    pub fn enqueue<P: Processor + 'a>(&mut self, processor: P) -> &mut Self {
        self.processors.push(Box::new(processor));
        self
    }
}
