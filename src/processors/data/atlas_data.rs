use std::{
    collections::HashMap,
    fs::{
        File,
        OpenOptions
    },
    io::{
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
