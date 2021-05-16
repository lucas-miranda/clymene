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
        GraphicSource
    },
    math::Size,
    processors::{
        ConfigStatus,
        packer::{
            CustomPacker,
            Packer
        },
        Processor,
        State
    },
    settings::Config
};

const DEFAULT_ATLAS_SIZE: u32 = 1024;

pub struct PackerProcessor {
    packer: Option<Box<dyn Packer>>
}

impl Processor for PackerProcessor {
    fn name(&self) -> &str {
        "Packer"
    }

    fn setup(&mut self, config: &mut Config) -> ConfigStatus {
        let mut config_status = ConfigStatus::NotModified;

        if config.packer.atlas_size == 0 {
            config.packer.atlas_size = DEFAULT_ATLAS_SIZE;
            config_status = ConfigStatus::Modified;
        }

        // TODO  make a better way to select packer
        self.packer = Some(Box::new(CustomPacker::new()));

        config_status
    }

    fn execute(&self, state: &mut State) {
        log::info!("-> Packing images...");

        match &self.packer {
            Some(packer) => {
                let mut graphic_sources = state.graphic_output.graphics
                                               .iter_mut()
                                               .filter_map(|g| -> Option<Box<dyn Iterator<Item = &mut GraphicSource>>> {
                                                   match g {
                                                       Graphic::Image(img) => Some(Box::new(iter::once(&mut img.graphic_source))),
                                                       Graphic::Animation(anim) => Some(Box::new(anim.frames.iter_mut().map(|f| &mut f.graphic_source))),
                                                       Graphic::Empty => None
                                                   }
                                               })
                                               .flatten()
                                               .collect::<Vec<&mut GraphicSource>>();

                packer.execute(Size::new(state.config.packer.atlas_size, state.config.packer.atlas_size), &mut graphic_sources);

                log::info!("-> Generating output...");

                let atlas_folder_path = state.config.cache.atlas_path();

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
                            _ => panic!("{}", e)
                        }
                    }
                }

                // generate atlas file

                let mut image_buffer = image::ImageBuffer::from_pixel(
                    state.config.packer.atlas_size, 
                    state.config.packer.atlas_size,
                    image::Rgba([0u8; 4])
                );

                for graphic_source in graphic_sources {
                    let image = image::open(&graphic_source.path).unwrap();

                    match &graphic_source.atlas_region {
                        Some(atlas_region) => {
                            //log::trace!("Copying graphic source {} to position {}", graphic_source.region, atlas_region);
                            image_buffer.copy_from(
                                &image.view(
                                    graphic_source.region.x, 
                                    graphic_source.region.y, 
                                    graphic_source.region.width, 
                                    graphic_source.region.height
                                ), 
                                atlas_region.x, 
                                atlas_region.y
                            ).unwrap();
                        },
                        None => {
                            log::warn!("Atlas region isn't defined from graphic source at '{}'", graphic_source.path.display());
                        }
                    }
                }

                let output_atlas_path = {
                    if state.config.output_name.is_empty() {
                        atlas_folder_path.join(format!("{}.png", Config::default_output_name()))
                    } else {
                        atlas_folder_path.join(format!("{}.png", state.config.output_name))
                    }
                };

                log::info!("Writing to file '{}'...", output_atlas_path.display());
                image_buffer.save_with_format(output_atlas_path, image::ImageFormat::Png).unwrap();
            },
            None => ()
        }
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
