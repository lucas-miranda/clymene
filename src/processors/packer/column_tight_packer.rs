use std::cmp::Ordering;

use crate::{
    graphics::GraphicSource,
    math::{
        Rectangle,
        Size,
        util
    },
    processors::packer::Packer
};

pub struct ColumnTightPacker {
}

impl Packer for ColumnTightPacker {
    fn name(&self) -> &str {
        "Custom"
    }

    fn execute(&self, atlas_size: Size<u32>, graphic_sources: &mut Vec<&mut GraphicSource>) -> Option<()> {
        if atlas_size.width == 0 || atlas_size.height == 0 {
            return None;
        }

        let a = atlas_size.clone();
        let mut empty_spaces: Vec<Rectangle<u32>> = vec![ atlas_size.into() ];

        // sort by decreasing order of their height
        graphic_sources.sort_by(|a, b| (*a).region.width.cmp(&(*b).region.width).reverse());

        for source in graphic_sources {
            if empty_spaces.is_empty() {
                panic!("Out of empty spaces.");
            }

            let empty_space = {
                let mut selected_space = None;
                let size = source.region.size();

                for space_index in 0..=empty_spaces.len() {
                    if empty_spaces[space_index].fit_size(&size) {
                        selected_space = Some(empty_spaces.remove(space_index));
                        break;
                    }
                }

                match selected_space {
                    Some(space) => space,
                    None => panic!("Can't find a valid location for source image '{}'.", source.path.display())
                }
            };

            let atlas_region = Rectangle::new(empty_space.x, empty_space.y, source.region.width, source.region.height);

            if empty_space.x + source.region.width > a.width {
                panic!("Source not fit. Source region: {}, Target Atlas Region: {}, Atlas size: {}, Empty space: {}", source.region, atlas_region, a, empty_space);
            }

            if empty_space.y + source.region.height > a.height {
                panic!("Source not fit. Source region: {}, Target Atlas Region: {}, Atlas size: {}, Empty space: {}", source.region, atlas_region, a, empty_space);
            }

            let space_right_side = if empty_space.width > source.region.width {
                Some(Rectangle::new(
                    atlas_region.right(), 
                    atlas_region.top(), 
                    empty_space.width - source.region.width,
                    empty_space.height
                ))
            } else {
                None
            };

            // space to the bottom
            let space_bottom_side = if empty_space.height > source.region.height {
                match &space_right_side {
                    Some(right_size) => {
                        Some(Rectangle::new(
                            atlas_region.left(), 
                            atlas_region.bottom(), 
                            empty_space.width - right_size.width,
                            empty_space.height - source.region.height
                        ))
                    },
                    None => {
                        Some(Rectangle::new(
                            atlas_region.left(), 
                            atlas_region.bottom(), 
                            empty_space.width,
                            empty_space.height - source.region.height
                        ))
                    }
                }
            } else {
                None
            };

            source.atlas_region = Some(atlas_region);

            // push spaces
            if let Some(space) = space_right_side {
                empty_spaces.push(space);
            }

            if let Some(space) = space_bottom_side {
                empty_spaces.push(space);
            }

            // sort empty spaces in descending order of size
            empty_spaces.sort_unstable_by(|a, b| {
                if (a.width > b.width && a.height > b.height) || util::max(&a.width, &a.height) > util::max(&b.width, &b.height) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
        }

        Some(())
    }
}

impl ColumnTightPacker {
    pub fn new() -> Self {
        Self {
        }
    }
}
