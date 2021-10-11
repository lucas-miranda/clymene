use colored::Colorize;
use image::{self, GenericImage};
use std::{
    fs, io, iter,
    path::{Path, PathBuf},
};
use tree_decorator::decorator;

use crate::{
    common::Verbosity,
    graphics::{animation::Frame, Graphic, GraphicSource},
    math::Size,
    modes::generator::processors::{output, ConfigStatus, Processor, State},
    settings::{Config, ProcessorConfig},
    util::Timer,
};

use super::{Packer, RowTightPacker};

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
            .join(format!("{}.png", config.output.name_or_default()))
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

    fn setup(&mut self, state: &mut State) -> ConfigStatus {
        let mut config_status = ConfigStatus::NotModified;

        infoln!(block, "Packing");

        if state.config.packer.atlas_size == 0 {
            state.config.packer.atlas_size = DEFAULT_ATLAS_SIZE;
            config_status = ConfigStatus::Modified;
        }

        // TODO  make a better way to select packer
        self.packer = Some(Box::new(RowTightPacker::new()));

        if state.args().global.force {
            state.graphic_output.request();
        } else {
            // check if will need to regenerate output file
            if !self.output_file_path(state.config).exists() {
                // ensure graphic output will be available at execute step
                state.graphic_output.request();
            }
        }

        doneln!();

        config_status
    }

    fn execute(&self, state: &mut State) {
        match &self.packer {
            Some(packer) => {
                infoln!(block, "Packing");
                let timer = Timer::start();

                if state.args().global.force {
                    infoln!(dashed, "Force Generate");
                } else {
                    infoln!(block, "Checking");

                    match &state.cache {
                        Some(c) => {
                            if c.is_updated() {
                                // output file
                                match state
                                    .output
                                    .register_file(&self.output_file_path(state.config))
                                {
                                    Ok(()) => {
                                        infoln!(last, "{}", "Already Updated".green());
                                        doneln!();
                                        return;
                                    }
                                    Err(e) => match e {
                                        output::Error::FileExpected => {
                                            infoln!("Output file not found")
                                        }
                                        _ => panic!("{}", e),
                                    },
                                }
                            }

                            infoln!(last, "{}", "Needs Update".blue());
                        }
                        None => panic!("Cache isn't available"),
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
                                Graphic::Animation(anim) => {
                                    Some(Box::new(anim.frames.iter_mut().filter_map(|f| match f {
                                        Frame::Empty => None,
                                        Frame::Contents { graphic_source, .. } => {
                                            Some(graphic_source)
                                        }
                                    })))
                                }
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
                state.output.register_file(&cache_output_path).unwrap();

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
