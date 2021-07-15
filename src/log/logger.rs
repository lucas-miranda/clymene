use std::collections::HashMap;

use super::LoggerModuleEntry;

#[derive(Default)]
pub struct Logger {
    modules: HashMap<String, LoggerModuleEntry>,
    verbose: bool,
    debug: bool
}

impl Logger {
    pub fn verbose(&mut self, value: bool) {
        self.verbose = value;
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub fn debug(&mut self, value: bool) {
        self.debug = value;
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }

    pub fn register_module<T: Into<String>>(&mut self, name: T, verbose: bool) {
        let mut submodule: Option<&mut LoggerModuleEntry> = None;

        for section_name in name.into().trim_start_matches(format!("{}::", env!("CARGO_PKG_NAME")).as_str()).split("::") {
            let section_name_owned = section_name.to_owned();

            submodule = match submodule {
                Some(m) => {
                    if m.contains_submodule(&section_name_owned) {
                        m.get_mut_submodule(&section_name_owned)
                    } else {
                        m.register_submodule(section_name, LoggerModuleEntry::default());
                        Some(m.get_mut_submodule(&section_name_owned).unwrap())
                    }
                },
                None => {
                    Some(match self.modules.get_mut(&section_name_owned) {
                        Some(module) => module,
                        None => {
                            self.modules.insert(section_name_owned.clone(), LoggerModuleEntry::default());
                            self.modules.get_mut(&section_name_owned).unwrap()
                        }
                    })
                }
            };
        }

        if let Some(last) = submodule {
            last.verbose = verbose;
        }
    }

    pub fn get_module<T: Into<String>>(&self, name: T) -> Option<&LoggerModuleEntry> {
        let mut submodule: Option<&LoggerModuleEntry> = None;

        for section_name in name.into().trim_start_matches(format!("{}::", env!("CARGO_PKG_NAME")).as_str()).split("::") {
            let section_name_owned = section_name.to_owned();

            submodule = match submodule {
                Some(m) => m.get_submodule(&section_name_owned),
                None => self.modules.get(&section_name_owned)
            };

            if submodule.is_none() {
                break;
            }
        }

        submodule
    }

    pub fn is_module_verbose<T: Into<String>>(&self, name: T) -> bool {
        if self.is_verbose() {
            return true;
        }

        let mut submodule: Option<&LoggerModuleEntry> = None;

        for section_name in name.into().trim_start_matches(format!("{}::", env!("CARGO_PKG_NAME")).as_str()).split("::") {
            let section_name_owned = section_name.to_owned();

            submodule = match submodule {
                Some(m) => m.get_submodule(&section_name_owned),
                None => self.modules.get(&section_name_owned)
            };

            match submodule {
                Some(m) => {
                    if m.is_verbose() {
                        return true;
                    }
                },
                None => break
            }
        }

        false
    }
}
