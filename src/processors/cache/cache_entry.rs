use std::{
    convert::TryInto,
    ffi::OsString,
    fs,
    path::{
        Path,
        PathBuf,
    },
    time::SystemTime
};

use log::error;

use serde::{ 
    Serialize, 
    Deserialize 
};

use crate::{
    graphics::{
        animation::{
            Animation,
            Frame,
            Track
        },
        Graphic,
        Image
    },
    math::{
        Rectangle,
        Size
    }
};

#[derive(Serialize, Deserialize)]
pub struct CacheEntry {
    pub modtime: SystemTime,

    #[serde(skip)]
    pub path: PathBuf
}

impl CacheEntry {
    pub fn retrieve_graphic<P: AsRef<Path>>(&self, source_path: P) -> Option<Graphic> {
        let mut source_images = Vec::new();
        let dir_iter;

        match fs::read_dir(&self.path) {
            Ok(iter) => dir_iter = iter,
            Err(_) => return None
        };

        // TODO  use a generic data format
        let data_pathbuf = self.path.join("data.json");
        let aseprite_data = match crate::processors::image::format_handlers::aseprite_handler::data::Data::from_file(&data_pathbuf) {
            Ok(data) => data,
            Err(_) => return None
        };

        // collect files' paths
        for dir_entry in dir_iter {
            if let Ok(entry) = dir_entry {
                let path = entry.path();

                if let Ok(metadata) = path.metadata() {
                    if !metadata.is_file() {
                        continue;
                    }

                    // frame index
                    let frame_index: u32 = match path.file_stem() {
                        Some(stem) => {
                            match stem.to_str() {
                                Some(stem_str) => {
                                    match stem_str.parse() {
                                        Ok(index) => index,
                                        Err(_) => continue
                                    }
                                },
                                None => continue
                            }
                        },
                        None => continue
                    };

                    let dimensions: Size<u32>;
                    let source_region: Rectangle<u32>;

                    match aseprite_data.frames.get(frame_index as usize) {
                        Some(frame_data) => {
                            dimensions = Size::with(frame_data.source_size.w, frame_data.source_size.h)
                                              .unwrap_or_else(Size::default);

                            source_region = Rectangle::with(
                                frame_data.sprite_source_size.x,
                                frame_data.sprite_source_size.y,
                                frame_data.sprite_source_size.w,
                                frame_data.sprite_source_size.h
                            ).unwrap_or_else(Rectangle::default);
                        },
                        None => {
                            dimensions = Size::default();
                            source_region = Rectangle::default();
                        }
                    }

                    match Image::new(path, source_path.as_ref().to_owned(), dimensions, source_region) {
                        Ok(image) => source_images.push(SourceImage { image, frame_index }),
                        Err(_) => ()
                    }
                    

                    /*
                    // unnamed track's frame
                    let unnamed_track = {
                        if let Some(track) = tracks.get_mut(&default_track_identifier) {
                            track
                        } else {
                            tracks.insert(default_track_identifier.clone(), Vec::new());
                            tracks.get_mut(&default_track_identifier).unwrap()
                        }
                    };

                    unnamed_track.push(file_name);
                    */
                }
            }
        }

        // build graphic

        if source_images.len() == 0 {
            return Some(Graphic::Empty);
        }

        if source_images.len() == 1 && aseprite_data.meta.frame_tags.len() == 0 && aseprite_data.meta.slices.len() == 0 {
            // single image
            return Some(source_images.remove(0).image.into());
        }

        let mut animation = match Animation::new(self.path.to_owned()) {
            Ok(anim) => anim,
            Err(_) => return None
        };

        // register source images
        for source_image in source_images.drain(..) {
            animation.insert_source_image(source_image.frame_index, source_image.image);
        }

        // register tracks
        for frame_tag_data in &aseprite_data.meta.frame_tags {
            let label = {
                if frame_tag_data.name.is_empty() {
                    None
                } else {
                    Some(frame_tag_data.name.clone())
                }
            };

            let mut track = Track::new(label);

            for index in frame_tag_data.from..(frame_tag_data.to + 1) {
                if index < 0 {
                    error!("Skipping invalid index '{}'.", index);
                    continue;
                }

                let index_u32 = index as u32;

                let duration = {
                    match index.try_into() {
                        Ok(i) => {
                            match aseprite_data.frames.get::<usize>(i) {
                                Some(frame_data) => frame_data.duration,
                                None => 0
                            }
                        },
                        Err(e) => {
                            error!("When trying to convert index type into usize.");
                            panic!(e);
                        }
                    }
                };

                track.push(Frame {
                    source_index: index_u32,
                    duration: {
                        if duration < 0 {
                            0
                        } else {
                            duration as u32
                        }
                    }
                });
            }

            animation.push_track(track);
        }

        Some(animation.into())
    }

    fn read_track_files(path: &PathBuf) -> Option<Vec<OsString>> {
        let sub_dir_iter;
        match fs::read_dir(&path) {
            Ok(iter) => sub_dir_iter = iter,
            Err(_) => return None
        };

        let mut frames = Vec::new();

        for dir_entry in sub_dir_iter {
            if let Ok(entry) = dir_entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        frames.push(entry.file_name())
                    }
                }
            }
        }

        if frames.len() > 0 {
            Some(frames)
        } else {
            None
        }
    }
}

struct SourceImage {
    pub image: Image,
    pub frame_index: u32
}
