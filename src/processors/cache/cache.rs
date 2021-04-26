use std::{
    cell::RefCell,
    cmp::Eq,
    collections::HashMap,
    fs::{
        File,
        Metadata,
        OpenOptions
    },
    io::{
        self,
        BufReader,
        BufWriter,
        Write
    },
    hash::Hash,
    path::{
        Path,
        PathBuf
    }
};

use serde::{ 
    Serialize, 
    Deserialize 
};

use super::{
    CacheEntry,
    CacheMetadata,
    CacheStatus,
    Error,
    LoadError,
    SaveError
};

#[derive(Serialize, Deserialize)]
pub struct Cache {
    #[serde(default)]
    pub meta: CacheMetadata,

    pub files: HashMap<PathBuf, RefCell<CacheEntry>>
}

impl Cache {
    pub fn new() -> Self {
        Self {
            meta: CacheMetadata::default(),
            files: HashMap::new()
        }
    }

    pub fn default_filename() -> &'static str {
        "cache.json"
    }

    pub fn load(file: &File) -> Result<Self, LoadError> {
        let buf_reader = BufReader::new(file);
        match serde_json::from_reader(buf_reader) {
            Ok(c) => Ok(c),
            Err(serde_json_error) => Err(LoadError::Deserialize(serde_json_error))
        }
    }

    pub fn load_from_path<P: AsRef<Path>>(filepath: P) -> Result<Self, LoadError> {
        match OpenOptions::new().read(true).open(&filepath) {
            Ok(file) => Self::load(&file),
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => Err(LoadError::FileNotFound(filepath.as_ref().to_owned())),
                    _ => panic!(e)
                }
            }
        }
    }

    pub fn save(&self, file: &mut File) -> Result<(), SaveError> {
        let mut buf_writer = BufWriter::new(file);

        serde_json::to_writer(&mut buf_writer, &self)
                   .map_err(SaveError::Serialize)?;

        buf_writer.flush().unwrap();

        Ok(())
    }

    pub fn save_to_path<P: AsRef<Path>>(&self, filepath: P) -> Result<(), SaveError> {
        let mut file = OpenOptions::new()
                                   .write(true)
                                   .append(false)
                                   .create(true)
                                   .open(filepath)
                                   .unwrap();

        self.save(&mut file)
    }

    pub fn retrieve<'r, P: AsRef<Path> + Eq + Hash>(&'r self, location: P, source_metadata: &Metadata) -> CacheStatus<'r> {
        let source_modtime = source_metadata.modified().unwrap();

        match self.files.get(location.as_ref()) {
            Some(cache_file) => {
                let cache = cache_file.borrow();
                if cache.modtime.eq(&source_modtime) {
                    CacheStatus::Found(cache)
                } else {
                    CacheStatus::Outdated
                }
            },
            None => CacheStatus::NotFound
        }
    }

    /*
    pub fn get<P: AsRef<Path> + Eq + Hash>(&self, location: P, source_metadata: &Metadata) -> Option<&RefCell<CacheEntry>> {
        self.files.get(location.as_ref())
    }
    */

    pub fn register<P: AsRef<Path> + Eq + Hash>(&mut self, location: P, metadata: &Metadata) -> Result<(), Error> {
        let modtime = metadata.modified().unwrap();

        self.files.insert(
            location.as_ref().to_owned(), 
            RefCell::new(CacheEntry {
                modtime,
                path: location.as_ref().to_owned()
            })
        );

        Ok(())
    }
}
