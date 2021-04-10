//use std::rc::Rc;
use std::cmp::Ordering;

use crate::{
    graphics::Image,
    math::{
        Rectangle,
        Size,
        util
    },
    processors::packer::Packer
};

pub struct CustomPacker {
}

impl Packer for CustomPacker {
    fn execute(&self, atlas_min_size: Size<u32>, source_images: &mut Vec<&mut Image>) -> Option<()> {
        if atlas_min_size.width == 0 || atlas_min_size.height == 0 {
            return None;
        }

        let atlas_size = atlas_min_size;
        let mut empty_spaces: Vec<Rectangle<u32>> = vec![ atlas_size.into() ];

        // sort by decreasing order of their height
        source_images.sort_by(|a, b| (*a).source_region.width.cmp(&(*b).source_region.width).reverse());

        for image in source_images {
            if empty_spaces.is_empty() {
                panic!("Out of empty spaces.");
            }

            let empty_space = {
                let mut selected_space = None;
                let size = image.source_region.size();

                for space_index in 0..(empty_spaces.len() + 1) {
                    if empty_spaces[space_index].fit_size(&size) {
                        selected_space = Some(empty_spaces.remove(space_index));
                        break;
                    }
                }

                match selected_space {
                    Some(space) => space,
                    None => panic!("Can't find a valid location for source image '{}'.", image.location.display())
                }
            };

            image.atlas_region = Rectangle::new(empty_space.x, empty_space.y, image.source_region.width, image.source_region.height);

            // space to the right
            let space_right_side = if empty_space.width - image.source_region.width > 0 {
                Some(Rectangle::new(
                    image.atlas_region.right(), 
                    image.atlas_region.top(), 
                    empty_space.width - image.source_region.width,
                    empty_space.height
                ))
            } else {
                None
            };

            // space to the bottom
            let space_bottom_side = if empty_space.height - image.source_region.height > 0 {
                match &space_right_side {
                    Some(right_size) => {
                        Some(Rectangle::new(
                            image.atlas_region.left(), 
                            image.atlas_region.bottom(), 
                            empty_space.width - right_size.width,
                            empty_space.height - image.source_region.height
                        ))
                    },
                    None => {
                        Some(Rectangle::new(
                            image.atlas_region.left(), 
                            image.atlas_region.bottom(), 
                            empty_space.width,
                            empty_space.height - image.source_region.height
                        ))
                    }
                }
            } else {
                None
            };

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

impl CustomPacker {
    pub fn new() -> Self {
        Self {
        }
    }
}
