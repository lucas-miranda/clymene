use std::collections::HashMap;

#[derive(Default)]
pub struct LoggerModuleEntry {
    pub(super) verbose: bool,
    submodules: HashMap<String, LoggerModuleEntry>
}

impl LoggerModuleEntry {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            submodules: HashMap::new()
        }
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub(super) fn register_submodule<T: Into<String>>(&mut self, name: T, entry: LoggerModuleEntry) {
        self.submodules.insert(name.into(), entry);
    }

    pub(super) fn get_submodule(&self, name: &String) -> Option<&LoggerModuleEntry> {
        self.submodules.get(name)
    }

    pub(super) fn get_mut_submodule(&mut self, name: &String) -> Option<&mut LoggerModuleEntry> {
        self.submodules.get_mut(name)
    }

    pub(super) fn contains_submodule(&self, name: &String) -> bool {
        self.submodules.contains_key(name)
    }
}
