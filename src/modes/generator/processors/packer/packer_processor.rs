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
    math::{self, Size},
    modes::generator::processors::{
        output::{self, AtlasOutputStats, OutputFile},
        ConfigStatus, Processor, State,
    },
    settings::{Config, ProcessorConfig},
    util::Timer,
};

use super::{Packer, PackerError, ValidationError};

const DEFAULT_MAX_ATLAS_SIZE: u32 = 4096;

pub struct PackerProcessor<P: Packer> {
    verbose: bool,
    packer: P,
}

impl<P: Packer> PackerProcessor<P> {
    pub fn new(packer: P) -> Self {
        PackerProcessor {
            verbose: false,
            packer,
        }
    }

    fn validate_output(&self, state: &mut State) -> Result<(), ValidationError> {
        let cache = state.cache.as_ref().expect("Cache isn't available");

        if !cache.is_updated() {
            return Err(ValidationError::CacheNotUpdated);
        }

        let config = state.config.try_read().expect("Can't retrieve a read lock");
        let output_filepath = self.output_file_path(&config);

        match output_filepath.metadata() {
            Ok(m) => {
                if m.is_file() {
                    // check image data
                    let (w, h) = image::image_dimensions(&output_filepath)
                        .map_err(ValidationError::AtlasImageLoadFailed)?;

                    if w != state.output.atlas_width || h != state.output.atlas_height {
                        traceln!(
                            "Previous output file image size {}x{} differs from current size {}x{}",
                            w,
                            h,
                            state.output.atlas_width,
                            state.output.atlas_height,
                        );

                        return Err(ValidationError::PreviousFileImageSizeMismatch);
                    }
                }
            }
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => (),
                _ => return Err(ValidationError::AtlasImageIoError(e)),
            },
        }

        let output_file = OutputFile::with_stats(output_filepath, AtlasOutputStats::new(0.0));

        if let Err(e) = state.output.register_file(output_file) {
            match e {
                output::Error::FileExpected => {
                    infoln!("Output file not found");
                    return Err(ValidationError::AtlasImageNotFound);
                }
                _ => panic!("{}", e),
            }
        }

        Ok(())
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
}

impl<P: Packer> Processor for PackerProcessor<P> {
    fn name(&self) -> &str {
        "Packer"
    }

    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> Option<&'a dyn ProcessorConfig> {
        Some(&config.packer)
    }

    fn setup(&mut self, state: &mut State) -> ConfigStatus {
        let mut config_status = ConfigStatus::NotModified;
        let mut c = state
            .config
            .try_write()
            .expect("Can't retrieve a write lock");

        infoln!(block, "Packing");

        if c.packer.atlas_size != 0 {
            if c.packer.optimize && !math::is_power_2(c.packer.atlas_size) {
                state
                    .output
                    .set_atlas_size(math::ceil_power_2(c.packer.atlas_size));

                traceln!(
                    "Optimizing atlas size from {}x{0} to {}x{}",
                    c.packer.atlas_size,
                    state.output.atlas_width,
                    state.output.atlas_height,
                );
            } else {
                state.output.atlas_width = c.packer.atlas_size;
                state.output.atlas_height = c.packer.atlas_size;
                traceln!(
                    "Using provided atlas size {}x{}",
                    state.output.atlas_width,
                    state.output.atlas_height
                );
            }
        } else {
            // store default value at config
            c.packer.atlas_size = state.output.atlas_width;
            config_status = ConfigStatus::Modified;

            traceln!("Using default atlas size {}x{0}", c.packer.atlas_size);
        }

        if state.args().global.force {
            state.graphic_output.request();
        } else {
            let output_filepath = self.output_file_path(&c);

            // check if will need to regenerate output file
            // and ensure graphic output will be available at execute step
            if output_filepath.is_file() {
                // check if output file differs from requested dimensions
                let (w, h) = image::image_dimensions(&output_filepath).unwrap_or_else(|_| {
                    panic!("Can't read output image at '{}'", output_filepath.display())
                });

                if state.output.atlas_width != w || state.output.atlas_height != h {
                    state.graphic_output.request();
                }
            } else if output_filepath.exists() {
                panic!(
                    "Output file path '{}' is already in use",
                    output_filepath.display()
                )
            } else {
                state.graphic_output.request();
            }
        }

        doneln!();

        config_status
    }

    fn execute(&mut self, state: &mut State) {
        infoln!(block, "Packing");
        let timer = Timer::start();

        if state.args().global.force {
            infoln!(dashed, "Force Generate");
        } else {
            infoln!(block, "Checking");

            match self.validate_output(state) {
                Ok(()) => {
                    infoln!(last, "{}", "Already Updated".green());
                    doneln!();
                    return;
                }
                Err(e) => match e {
                    ValidationError::CacheNotUpdated
                    | ValidationError::AtlasImageNotFound
                    | ValidationError::PreviousFileImageSizeMismatch => (),
                    _ => panic!("{}", e),
                },
            }

            infoln!(last, "{}", "Needs Update".blue());
        }

        infoln!(block, "Calculating");
        traceln!(
            entry: decorator::Entry::None,
            "With atlas size {}x{}",
            state.output.atlas_width,
            state.output.atlas_height,
        );

        let mut graphic_sources = state
            .graphic_output
            .graphics
            .iter_mut()
            .filter_map(
                |g| -> Option<Box<dyn Iterator<Item = &mut GraphicSource>>> {
                    match g {
                        Graphic::Image(img) => Some(Box::new(iter::once(&mut img.graphic_source))),
                        Graphic::Animation(anim) => {
                            Some(Box::new(anim.frames.iter_mut().filter_map(|f| match f {
                                Frame::Empty => None,
                                Frame::Contents { graphic_source, .. } => Some(graphic_source),
                            })))
                        }
                        Graphic::Empty => None,
                    }
                },
            )
            .flatten()
            .collect::<Vec<&mut GraphicSource>>();

        infoln!("Using {} packer", self.packer.name().bold());
        let c = state.config.try_read().expect("Can't retrieve a read lock");

        let until_atlas_size = if c.packer.retry.until_atlas_size == 0 {
            DEFAULT_MAX_ATLAS_SIZE
        } else {
            c.packer.retry.until_atlas_size
        };

        let mut retries = 0;
        let usage = loop {
            match self.packer.execute(
                Size::new(state.output.atlas_width, state.output.atlas_height),
                &mut graphic_sources,
            ) {
                Ok(u) => break u,
                Err(err) => {
                    let can_retry = {
                        match err {
                            PackerError::EmptyTargetSize => false,
                            PackerError::OutOfSpace => {
                                c.packer.retry.enable
                                    && (c.packer.retry.max_retries == 0
                                        || retries < c.packer.retry.max_retries)
                                    && !(state.output.atlas_width == until_atlas_size
                                        && state.output.atlas_height == until_atlas_size)
                            }
                        }
                    };

                    if !can_retry {
                        panic!(
                            "Packer '{}' can't complete it's execution successfully.\n{}",
                            self.packer.name(),
                            err
                        )
                    }

                    if state.output.atlas_width < until_atlas_size {
                        state.output.atlas_width =
                            (state.output.atlas_width * 2).min(until_atlas_size);
                    }

                    if state.output.atlas_height < until_atlas_size {
                        state.output.atlas_height =
                            (state.output.atlas_height * 2).min(until_atlas_size);
                    }

                    retries += 1;

                    errorln!(block, "{}", "Can't complete".red());
                    errorln!(last, "{}", err);
                    infoln!(entry: decorator::Entry::None);
                    infoln!(block, "Retry #{}", retries.to_string().bold());
                    infoln!(
                        last,
                        "With atlas size: {}x{}",
                        state.output.atlas_width,
                        state.output.atlas_height
                    );
                }
            }
        };

        infoln!(last, "{}", "Done".green());
        infoln!("Generating output");

        let atlas_dir_path = c.cache.atlas_path();
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
        let cache_output_path = self.output_file_path(&c);

        infoln!(
            "Exporting to file {}",
            cache_output_path.display().to_string().bold()
        );

        self.generate_image(
            &cache_output_path,
            state.output.atlas_width,
            state.output.atlas_height,
            &graphic_sources,
        )
        .unwrap();

        // output
        let output_file = OutputFile::with_stats(cache_output_path, AtlasOutputStats::new(usage));
        state.output.register_file(output_file).unwrap();

        doneln_with_timer!(timer);
    }
}

impl<P: Packer> Verbosity for PackerProcessor<P> {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
