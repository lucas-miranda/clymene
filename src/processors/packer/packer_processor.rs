use std::{
    fs,
    io,
    iter
};

use image::{
    self,
    GenericImage,
    GenericImageView
};

use crate::{
    graphics::{
        Graphic,
        Image
    },
    math::Size,
    processors::{
        ConfigStatus,
        Data,
        Error,
        packer::{
            CustomPacker,
            Packer
        },
        Processor
    },
    settings::Config
};

const DEFAULT_ATLAS_SIZE: u32 = 1024;

pub struct PackerProcessor {
    packer: Option<Box<dyn Packer>>
}

impl Processor for PackerProcessor {
    fn setup(&mut self, config: &mut Config) -> Result<ConfigStatus, Error> {
        let mut config_status = ConfigStatus::NotModified;

        if config.packer.atlas_size == 0 {
            config.packer.atlas_size = DEFAULT_ATLAS_SIZE;
            config_status = ConfigStatus::Modified;
        }

        // TODO  make a better way to select packer
        self.packer = Some(Box::new(CustomPacker::new()));

        Ok(config_status)
    }

    fn execute(&self, data: &mut Data) -> Result<(), Error> {
        log::info!("-> Packing images...");

        match &self.packer {
            Some(packer) => {
                let mut source_images = data.graphic_output.graphics
                                            .iter_mut()
                                            .filter_map(|g| -> Option<Box<dyn Iterator<Item = &mut Image>>> {
                                                match g {
                                                    Graphic::Image(img) => Some(Box::new(iter::once(img))),
                                                    Graphic::Animation(anim) => Some(Box::new(anim.source_images.values_mut())),
                                                    Graphic::Empty => None
                                                }
                                            })
                                            .flatten()
                                            .collect::<Vec<&mut Image>>();

                packer.execute(Size::new(data.config.packer.atlas_size, data.config.packer.atlas_size), &mut source_images);

                log::info!("-> Generating output...");

                let atlas_folder_path = data.config.cache.atlas_path();

                // ensure atlas folder path it's valid
                match atlas_folder_path.metadata() {
                    Ok(metadata) => {
                        if !metadata.is_dir() {
                            panic!("Atlas output path '{}' is already in use.", atlas_folder_path.display());
                        }
                    },
                    Err(e) => {
                        match e.kind() {
                            io::ErrorKind::NotFound => {
                                fs::create_dir(&atlas_folder_path).unwrap();
                            },
                            _ => panic!(e)
                        }
                    }
                }

                // generate atlas file

                let mut image_buffer = image::ImageBuffer::from_pixel(
                    data.config.packer.atlas_size, 
                    data.config.packer.atlas_size,
                    image::Rgba([0u8; 4])
                );

                for source_image in source_images {
                    let image = image::open(&source_image.location).unwrap();
                    image_buffer.copy_from(
                        &image.view(
                            source_image.source_region.x, 
                            source_image.source_region.y, 
                            source_image.source_region.width, 
                            source_image.source_region.height
                        ), 
                        source_image.atlas_region.x, 
                        source_image.atlas_region.y
                    ).unwrap();
                }

                let output_atlas_path = {
                    if data.config.output_name.is_empty() {
                        atlas_folder_path.join(format!("{}.png", Config::default_output_name()))
                    } else {
                        atlas_folder_path.join(format!("{}.png", data.config.output_name))
                    }
                };

                log::info!("Writing to file '{}'...", output_atlas_path.display());
                image_buffer.save_with_format(output_atlas_path, image::ImageFormat::Png).unwrap();
            },
            None => ()
        }

        /*
        // !!! REMOVE ME !!!
        log::info!("Packer Result:");

        for image in &data.graphic_output.images {
            log::info!("{:?}", image);
        }

        for anim in &data.graphic_output.animations {
            log::info!("{:?}", anim);
        }
        // !!! REMOVE ME !!!
        */

        Ok(())
    }
}

impl PackerProcessor {
    pub fn new() -> Self {
        PackerProcessor {
            packer: None
        }
    }

    fn calculate_optimal_atlas_size(size: u32) -> u32 {
        // round up to the next highest power of 2
        // ref: https://graphics.stanford.edu/~seander/bithacks.html#RoundUpPowerOf2

        if size == 0u32 {
            return DEFAULT_ATLAS_SIZE;
        }

        let mut size = size - 1;
        size |= size >> 1;
        size |= size >> 2;
        size |= size >> 4;
        size |= size >> 8;
        size |= size >> 16;

        size + 1
    }

}
