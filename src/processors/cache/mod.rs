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

mod cache_entry;
pub use cache_entry::CacheEntry;

mod cache_importer_processor;
pub use cache_importer_processor::CacheImporterProcessor;

mod cache_exporter_processor;
pub use cache_exporter_processor::CacheExporterProcessor;

mod cache_metadata;
pub use cache_metadata::CacheMetadata;

mod cache_status;
pub use cache_status::CacheStatus;

mod error;
pub use error::{
    Error,
    LoadError,
    SaveError
};

use crate::processors::data::GraphicData;

#[derive(Serialize, Deserialize)]
pub struct Cache {
    #[serde(default)]
    pub meta: CacheMetadata,

    /// Cache entries by location.
    pub files: HashMap<PathBuf, RefCell<CacheEntry>>,

    #[serde(skip)]
    pub images_path: PathBuf,

    #[serde(skip)]
    pub atlas_output_path: PathBuf,

    #[serde(skip)]
    outdated: bool
}

impl Cache {
    pub fn new(images_path: PathBuf, atlas_output_path: PathBuf) -> Self {
        Self {
            meta: CacheMetadata::default(),
            files: HashMap::new(),
            images_path,
            atlas_output_path,
            outdated: true
        }
    }

    pub fn default_filename() -> &'static str {
        "cache.json"
    }

    pub fn load<P: Into<PathBuf>>(file: &File, images_path: P, atlas_output_path: P) -> Result<Self, LoadError> {
        let buf_reader = BufReader::new(file);
        match serde_json::from_reader::<_, Cache>(buf_reader) {
            Ok(mut c) => {
                c.images_path = images_path.into();
                c.atlas_output_path = atlas_output_path.into();

                // fill location field of every cache entry
                for (location, cache_ref) in c.files.iter_mut() {
                    cache_ref.get_mut().location = location.clone();
                }

                Ok(c)
            },
            Err(serde_json_error) => Err(LoadError::Deserialize(serde_json_error))
        }
    }

    pub fn load_from_path<P: AsRef<Path>, T: Into<PathBuf>>(filepath: P, images_path: T, atlas_output_path: T) -> Result<Self, LoadError> {
        match OpenOptions::new().read(true).open(&filepath) {
            Ok(file) => Self::load(&file, images_path, atlas_output_path),
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => Err(LoadError::FileNotFound(filepath.as_ref().to_owned())),
                    _ => panic!("{}", e)
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

    pub fn register<P: AsRef<Path> + Eq + Hash>(&mut self, location: P, metadata: &Metadata, data: GraphicData) -> Result<(), Error> {
        let modtime = metadata.modified().unwrap();

        self.files.insert(
            location.as_ref().to_owned(), 
            RefCell::new(CacheEntry {
                modtime,
                data,
                location: location.as_ref().to_owned()
            })
        );

        Ok(())
    }

    pub fn is_outdated(&self) -> bool {
        self.outdated
    }

    pub fn is_updated(&self) -> bool {
        !self.outdated
    }

    pub fn mark_as_outdated(&mut self) {
        if self.is_outdated() {
            return;
        }

        self.outdated = true;
    }
}
