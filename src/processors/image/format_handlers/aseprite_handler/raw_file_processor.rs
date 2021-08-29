use asefile::AsepriteFile;
use image;
use std::path::Path;

use crate::{
    graphics::{
        animation::{Animation, Track},
        Graphic, GraphicSource, Image,
    },
    math::Rectangle,
    processors::{
        image::format_handlers::{Error, FormatProcessor},
        ConfigStatus,
    },
    settings::Config,
};

#[derive(Default)]
pub struct RawFileProcessor {}

impl FormatProcessor for RawFileProcessor {
    fn setup(&self, _config: &mut Config) -> Result<ConfigStatus, Error> {
        Ok(ConfigStatus::NotModified)
    }

    fn process(
        &self,
        source_file_path: &Path,
        output_dir_path: &Path,
        _config: &Config,
    ) -> Result<Graphic, Error> {
        let ase = AsepriteFile::read_file(source_file_path).unwrap();
        let frame_count = ase.num_frames();

        match frame_count {
            0 => Ok(Graphic::Empty),
            1 => {
                let frame_image_buffer = ase.frame(0).image();
                let frame_output_path = output_dir_path.join("0.png");
                frame_image_buffer
                    .save_with_format(&frame_output_path, image::ImageFormat::Png)
                    .unwrap();

                let (w, h) = frame_image_buffer.dimensions();
                let graphic_source =
                    GraphicSource::new(frame_output_path, Rectangle::new(0, 0, w, h));
                let image = Image::with_graphic_source(graphic_source, source_file_path.to_owned())
                    .unwrap();

                Ok(image.into())
            }
            _ => {
                let mut animation = Animation::new(source_file_path.to_owned())
                    .map_err::<Error, _>(|e| e.into())?;

                // frames
                for frame_index in 0..frame_count {
                    let frame = ase.frame(frame_index);
                    let frame_image_buffer = frame.image();
                    let frame_output_path = output_dir_path.join(format!("{}.png", frame_index));

                    frame_image_buffer
                        .save_with_format(&frame_output_path, image::ImageFormat::Png)
                        .unwrap();

                    let (w, h) = frame_image_buffer.dimensions();

                    // ensure w and h isn't zero
                    if w == 0 || h == 0 {
                        continue;
                    }

                    let mut source: Option<Rectangle<u32>> = None;

                    for (row, row_pixels) in frame_image_buffer.enumerate_rows() {
                        let mut start_column = None;
                        let mut end_column = None;

                        for (x, _y, px) in row_pixels {
                            let alpha = px[3];

                            // alpha must be at least 1% (0.01 * 255 ~= 3)
                            if alpha >= 3u8 {
                                if start_column.is_none() {
                                    start_column = Some(x);
                                }

                                end_column = Some(x);
                            }
                        }

                        if let Some(s) = &mut source {
                            if let Some(start) = start_column {
                                if start < s.left() {
                                    s.set_left(start);
                                }

                                // only tries to update row if a pixel was found
                                if row > s.bottom() {
                                    s.set_bottom(row);
                                }
                            }

                            if let Some(end) = end_column {
                                if end > s.right() {
                                    s.set_right(end);
                                }
                            }
                        } else if let Some(start) = start_column {
                            source =
                                Some(Rectangle::with_bounds(start, row, end_column.unwrap(), row));
                        }
                    }

                    let graphic_source = GraphicSource::new(
                        frame_output_path,
                        match source {
                            Some(s) => s,
                            None => source.unwrap_or_else(Rectangle::default),
                        },
                    );

                    animation.push_frame(graphic_source, frame.duration())
                }

                // tags
                let tag_count = ase.num_tags();

                for tag_index in 0..tag_count {
                    let tag = ase.tag(tag_index);
                    let mut track = Track::new(Some(tag.name().to_owned()));

                    for frame_index in tag.from_frame()..=tag.to_frame() {
                        track.frame_indices.push(frame_index);
                    }

                    animation.push_track(track);
                }

                Ok(animation.into())
            }
        }
    }
}
