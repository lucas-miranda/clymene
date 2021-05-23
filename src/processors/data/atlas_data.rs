use std::{
    collections::HashMap,
    fs::{
        File,
        OpenOptions
    },
    io::{
        self,
        BufReader,
        BufWriter,
        Write
    },
    path::Path
};

use serde::{
    Deserialize,
    Serialize
};

use super::{
    GraphicData,
    LoadError,
    MetaData,
    SaveError
};

#[derive(Serialize, Deserialize)]
pub struct AtlasData {
    pub graphics: HashMap<String, GraphicData>,
    pub meta: MetaData
}

impl AtlasData {
    pub fn new() -> Self {
        Self {
            graphics: HashMap::new(),
            meta: MetaData::new()
        }
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

    pub fn save_pretty(&self, file: &mut File) -> Result<(), SaveError> {
        let mut buf_writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut buf_writer, &self)
                   .map_err(SaveError::Serialize)?;

        buf_writer.flush().unwrap();

        Ok(())
    }

    pub fn save_pretty_to_path<P: AsRef<Path>>(&self, filepath: P) -> Result<(), SaveError> {
        let mut file = OpenOptions::new()
                                   .write(true)
                                   .append(false)
                                   .create(true)
                                   .open(filepath)
                                   .unwrap();

        self.save_pretty(&mut file)
    }
}
