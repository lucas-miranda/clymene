use std::vec::Vec;

use crate::{
    args::Args,
    processors::{
        ConfigStatus,
        Data,
        Processor
    },
    settings::Config
};

pub struct ProcessorsPipeline<'a> {
    processors: Vec<Box<dyn Processor + 'a>>
}

impl<'a> ProcessorsPipeline<'a> {
    pub fn new() -> Self {
        Self {
            processors: Vec::new()
        }
    }

    pub fn start(&mut self, config: &mut Config, args: &Args) {
        let mut config_status = ConfigStatus::NotModified;

        for processor in self.processors.iter_mut() {
            match &processor.setup(config) {
                ConfigStatus::NotModified => (),
                ConfigStatus::Modified => {
                    // save processor config status for later use
                    // if it's modified
                    match &config_status {
                        ConfigStatus::NotModified => config_status = ConfigStatus::Modified,
                        ConfigStatus::Modified => ()
                    }
                }
            }
        }

        if let ConfigStatus::Modified = config_status {
            // config was modified, we need to save it to keep updated
            config.save_to_path(&args.config_filepath)
                  .unwrap();
        }

        let mut data = Data::new(config);

        for processor in &self.processors {
            processor.execute(&mut data);
        }
    }

    pub fn enqueue<P: Processor + 'a>(&mut self, processor: P) -> &mut Self {
        self.processors.push(Box::new(processor));
        self
    }
}
