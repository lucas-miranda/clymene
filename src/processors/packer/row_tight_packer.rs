use std::cmp::Ordering;

use crate::{
    graphics::GraphicSource,
    math::{util, Rectangle, Size},
    processors::packer::Packer,
};

pub struct RowTightPacker {}

impl RowTightPacker {
    pub fn new() -> Self {
        Self {}
    }
}

impl Packer for RowTightPacker {
    fn name(&self) -> &str {
        "Row Tight"
    }

    fn execute(
        &self,
        atlas_size: Size<u32>,
        graphic_sources: &mut Vec<&mut GraphicSource>,
    ) -> Option<()> {
        if atlas_size.width == 0 || atlas_size.height == 0 {
            return None;
        }

        let a = atlas_size.clone();
        let mut empty_spaces: Vec<Rectangle<u32>> = vec![atlas_size.into()];

        // sort by increasing order of their height and width
        graphic_sources.sort_unstable_by(|a, b| {
            (*a).region.height
                .cmp(&(*b).region.height)
                .then((*a).region.width.cmp(&(*b).region.width))
        });

        // reverse traverse sorted graphic sources
        for source in graphic_sources.iter_mut().rev() {
            if empty_spaces.is_empty() {
                panic!("Out of empty spaces.");
            }

            let empty_space = {
                let size = source.region.size();
                let mut best_fit: Option<SpaceFit> = None;

                for (space_index, empty_space) in empty_spaces.iter().filter(|s| s.fit_size(&size)).enumerate() {
                    if let Some(best) = &mut best_fit {
                        let extra_width = empty_space.width - size.width;
                        let extra_height = empty_space.height - size.height;

                        // try to fit at most top-left valid empty space
                        if empty_space.y.cmp(&best.y).then(empty_space.x.cmp(&best.x)).is_le() || extra_width == 0 || extra_height == 0 {
                            best.index = space_index;
                            best.y = empty_space.y;
                        }
                    } else {
                        best_fit = Some(SpaceFit {
                            index: space_index,
                            x: empty_space.x,
                            y: empty_space.y,
                        });
                    }
                }

                match best_fit {
                    Some(space) => empty_spaces.remove(space.index),
                    None => panic!(
                        "Can't find a valid location for source image '{}'.",
                        source.path.display()
                    ),
                }
            };

            let atlas_region = Rectangle::new(
                empty_space.x,
                empty_space.y,
                source.region.width,
                source.region.height,
            );

            // choose the best split, horizontal or vertical, to maximize sub areas
            if empty_space.bottom() - atlas_region.bottom() <= source.region.height {
                // vertical slice
                // * right empty area will be maximized

                if empty_space.width > source.region.width {
                    empty_spaces.push(Rectangle::new(
                        atlas_region.right(),
                        atlas_region.top(),
                        empty_space.width - source.region.width,
                        empty_space.height,
                    ));
                }

                if empty_space.height > source.region.height {
                    empty_spaces.push(Rectangle::new(
                        atlas_region.left(),
                        atlas_region.bottom(),
                        source.region.width,
                        empty_space.height - source.region.height,
                    ));
                }
            } else {
                // horizontal slice
                // * bottom empty area will be maximized

                if empty_space.width > source.region.width {
                    empty_spaces.push(Rectangle::new(
                        atlas_region.right(),
                        atlas_region.top(),
                        empty_space.width - source.region.width,
                        source.region.height,
                    ));
                }

                if empty_space.height > source.region.height {
                    empty_spaces.push(Rectangle::new(
                        atlas_region.left(),
                        atlas_region.bottom(),
                        empty_space.width,
                        empty_space.height - source.region.height,
                    ));
                }
            }

            source.atlas_region = Some(atlas_region);
        }

        Some(())
    }
}

struct SpaceFit {
    pub index: usize,
    pub x: u32,
    pub y: u32,
}
