use colored::Colorize;
use image::{self, GenericImage};
use std::{
    fs, io, iter,
    path::{Path, PathBuf},
};
use tree_decorator::decorator;

use crate::{
    common::Verbosity,
    graphics::{Graphic, GraphicSource},
    math::Size,
    modes::generator::processors::{ConfigStatus, Processor, State},
    settings::{Config, ProcessorConfig},
    util::Timer,
};

use super::{Error, Packer, RowTightPacker};

const DEFAULT_ATLAS_SIZE: u32 = 1024;

pub struct PackerProcessor {
    verbose: bool,
    packer: Option<Box<dyn Packer>>,
}

impl PackerProcessor {
    pub fn new() -> Self {
        PackerProcessor {
            verbose: false,
            packer: None,
        }
    }

    fn should_generate(&self, state: &State) -> Result<bool, Error> {
        if let Some(cache) = &state.cache {
            // verify if exists files at output path
            let output_file_name = self.output_file_path(state.config);
            match output_file_name.metadata() {
                Ok(metadata) => {
                    if !metadata.is_file() {
                        traceln!(
                            entry: decorator::Entry::None,
                            "Expected output file {} isn't a file",
                            output_file_name.display().to_string().bold()
                        );
                        return Err(Error::OutputFilepathAlreadyInUse(output_file_name));
                    }
                }
                Err(e) => {
                    if let Some(parent_path) = output_file_name.parent() {
                        if !parent_path.exists() {
                            return Err(Error::InvalidOutputDirectoryPath(parent_path.to_owned()));
                        }
                    }

                    if let io::ErrorKind::NotFound = e.kind() {
                        traceln!(
                            entry: decorator::Entry::None,
                            "Output file {} doesn't seems to exists",
                            output_file_name.display().to_string().bold()
                        );
                        return Ok(true);
                    }

                    return Err(Error::IO(e));
                }
            }

            // check if cache still is updated
            if cache.is_updated() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn generate_image(
        &self,
        output_path: &Path,
        width: u32,
        height: u32,
        graphic_sources: &[&mut GraphicSource],
    ) -> Result<(), image::ImageError> {
        let mut image_buffer = image::ImageBuffer::from_pixel(width, height, image::Rgba([0u8; 4]));

        for graphic_source in graphic_sources {
            match &graphic_source.atlas_region {
                Some(atlas_region) => {
                    image_buffer.copy_from(
                        &graphic_source.region_buffer_view(),
                        atlas_region.x,
                        atlas_region.y,
                    )?;
                }
                None => {
                    warnln!("Atlas region isn't defined at graphic source");
                }
            }
        }

        image_buffer.save_with_format(output_path, image::ImageFormat::Png)
    }

    fn output_file_path(&self, config: &Config) -> PathBuf {
        config
            .cache
            .atlas_path()
            .join(if config.output_name.is_empty() {
                format!("{}.png", Config::default_output_name())
            } else {
                format!("{}.png", config.output_name)
            })
    }

    /*
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
    */
}

impl Processor for PackerProcessor {
    fn name(&self) -> &str {
        "Packer"
    }

    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> Option<&'a dyn ProcessorConfig> {
        Some(&config.packer)
    }

    fn setup(&mut self, config: &mut Config) -> ConfigStatus {
        let mut config_status = ConfigStatus::NotModified;

        if config.packer.atlas_size == 0 {
            config.packer.atlas_size = DEFAULT_ATLAS_SIZE;
            config_status = ConfigStatus::Modified;
        }

        // TODO  make a better way to select packer
        self.packer = Some(Box::new(RowTightPacker::new()));

        config_status
    }

    fn execute(&self, state: &mut State) {
        match &self.packer {
            Some(packer) => {
                infoln!(block, "Packing");
                let timer = Timer::start();

                if state.force {
                    infoln!(dashed, "Force Generate");
                } else {
                    infoln!(block, "Checking");

                    if !self.should_generate(state).unwrap() {
                        // output
                        state
                            .output
                            .register_file(&self.output_file_path(state.config));

                        infoln!(last, "{}", "Already Updated".green());
                        infoln!(last, "{}", "Done".green());
                        return;
                    } else {
                        infoln!(last, "{}", "Needs Update".blue());
                    }
                }

                infoln!(block, "Calculating");
                traceln!(
                    entry: decorator::Entry::None,
                    "With atlas size {}x{}",
                    state.config.packer.atlas_size,
                    state.config.packer.atlas_size
                );

                let mut graphic_sources = state
                    .graphic_output
                    .graphics
                    .iter_mut()
                    .filter_map(
                        |g| -> Option<Box<dyn Iterator<Item = &mut GraphicSource>>> {
                            match g {
                                Graphic::Image(img) => {
                                    Some(Box::new(iter::once(&mut img.graphic_source)))
                                }
                                Graphic::Animation(anim) => Some(Box::new(
                                    anim.frames.iter_mut().map(|f| &mut f.graphic_source),
                                )),
                                Graphic::Empty => None,
                            }
                        },
                    )
                    .flatten()
                    .collect::<Vec<&mut GraphicSource>>();

                traceln!("Using {} packer", packer.name().bold());
                packer.execute(
                    Size::new(
                        state.config.packer.atlas_size,
                        state.config.packer.atlas_size,
                    ),
                    &mut graphic_sources,
                );

                infoln!(last, "{}", "Done".green());
                infoln!("Generating output");

                let atlas_dir_path = state.config.cache.atlas_path();
                traceln!(
                    entry: decorator::Entry::None,
                    "With output path {}",
                    atlas_dir_path.display().to_string().bold()
                );

                // ensure atlas directory path is valid
                match atlas_dir_path.metadata() {
                    Ok(metadata) => {
                        if !metadata.is_dir() {
                            panic!(
                                "Atlas output path '{}' is already in use.",
                                atlas_dir_path.display()
                            );
                        }
                    }
                    Err(e) => match e.kind() {
                        io::ErrorKind::NotFound => {
                            traceln!("Creating output directory");
                            fs::create_dir(&atlas_dir_path).unwrap();
                        }
                        _ => panic!("{}", e),
                    },
                }

                // generate atlas file at cache output path
                let cache_output_path = self.output_file_path(state.config);

                infoln!(
                    "Exporting to file {}",
                    cache_output_path.display().to_string().bold()
                );

                self.generate_image(
                    &cache_output_path,
                    state.config.packer.atlas_size,
                    state.config.packer.atlas_size,
                    &graphic_sources,
                )
                .unwrap();

                // output
                state.output.register_file(&cache_output_path);

                doneln_with_timer!(timer);
            }
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
