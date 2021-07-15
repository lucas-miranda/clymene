use std::{
    fs,
    io,
    iter
};

use colored::Colorize;

use image::{
    self,
    GenericImage,
    GenericImageView
};

use tree_decorator::decorator;

use crate::{
    common::Verbosity,
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
    settings::{
        Config,
        ProcessorConfig
    }
};

const DEFAULT_ATLAS_SIZE: u32 = 1024;

pub struct PackerProcessor {
    verbose: bool,
    packer: Option<Box<dyn Packer>>
}

impl PackerProcessor {
    pub fn new() -> Self {
        PackerProcessor {
            verbose: false,
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

impl Processor for PackerProcessor {
    fn name(&self) -> &str {
        "Packer"
    }

    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> &'a dyn ProcessorConfig {
        &config.packer
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
        match &self.packer {
            Some(packer) => {
                infoln!(block, "Packing");
                infoln!(block, "Calculating");
                traceln!(entry: decorator::Entry::None, "With atlas size {}x{}", state.config.packer.atlas_size, state.config.packer.atlas_size);

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

                traceln!("Using {} packer", packer.name().bold());
                packer.execute(Size::new(state.config.packer.atlas_size, state.config.packer.atlas_size), &mut graphic_sources);

                infoln!(last, "{}", "Done".green());
                infoln!("Generating output");

                let atlas_dir_path = state.config.cache.atlas_path();
                traceln!(entry: decorator::Entry::None, "With output path {}", atlas_dir_path.display().to_string().bold());

                // ensure atlas directory path is valid
                match atlas_dir_path.metadata() {
                    Ok(metadata) => {
                        if !metadata.is_dir() {
                            panic!("Atlas output path '{}' is already in use.", atlas_dir_path.display());
                        }
                    },
                    Err(e) => {
                        match e.kind() {
                            io::ErrorKind::NotFound => {
                                traceln!("Creating output directory");
                                fs::create_dir(&atlas_dir_path).unwrap();
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
                            warnln!("Atlas region isn't defined from graphic source at '{}'", graphic_source.path.display());
                        }
                    }
                }

                let output_atlas_path = {
                    if state.config.output_name.is_empty() {
                        atlas_dir_path.join(format!("{}.png", Config::default_output_name()))
                    } else {
                        atlas_dir_path.join(format!("{}.png", state.config.output_name))
                    }
                };

                infoln!("Exporting to file {}", output_atlas_path.display().to_string().bold());
                image_buffer.save_with_format(output_atlas_path, image::ImageFormat::Png).unwrap();

                infoln!(last, "{}", "Done".green());
            },
            None => {
                warnln!("There is no packer defined");
            }
        }
    }
}

impl Verbosity for PackerProcessor {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
