use colored::Colorize;

use crate::{
    common::Verbosity,
    modes::generator::processors::{ConfigStatus, Processor, State},
    settings::{Config, OutputConfig, ProcessorConfig},
};

pub struct ConfigProcessor {
    verbose: bool,
}

impl ConfigProcessor {
    pub fn new() -> Self {
        ConfigProcessor { verbose: false }
    }
}

impl Processor for ConfigProcessor {
    fn name(&self) -> &str {
        "Config"
    }

    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> Option<&'a dyn ProcessorConfig> {
        Some(config)
    }

    fn setup(&mut self, config: &mut Config) -> ConfigStatus {
        let mut config_status = ConfigStatus::NotModified;

        infoln!(
            block,
            "Checking {} config",
            env!("CARGO_PKG_NAME").bold().magenta()
        );

        // output name
        if config.output.name.is_empty() {
            config.output.name = OutputConfig::default_name();
            config_status = ConfigStatus::Modified;
        }

        infoln!(last, "{}", "Ok".green());

        config_status
    }

    fn execute(&self, _state: &mut State) {
        // there is nothing to do at this phase
    }
}

impl Verbosity for ConfigProcessor {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
