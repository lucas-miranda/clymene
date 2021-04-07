//use std::rc::Rc;
use std::cmp::Ordering;

use crate::{
    graphics::Image,
    math::{
        Rectangle,
        Size
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

        let atlas_size = atlas_min_size.clone();

        let mut empty_spaces: Vec<Rectangle<u32>> = vec![ atlas_size.clone().into() ];

        // sort by decreasing order of their height
        source_images.sort_by(|a, b| (*b).source_region.height.cmp(&(*a).source_region.height));

        for source_image in source_images {
            let img = source_image;
            let size = img.source_region.size();

            if empty_spaces.len() == 0 {
                panic!("Out of empty spaces.");
            }

            let first_empty_space = empty_spaces.remove(0);

            if !first_empty_space.fit_size(&size) {
                panic!("Out of space to fit source image.");
            }

            img.atlas_region = Rectangle::new(first_empty_space.x, first_empty_space.y, size.width, size.height);

            // space to the right
            if atlas_size.width - (first_empty_space.x + size.width) > 0 {
                empty_spaces.push(Rectangle::new(
                    img.atlas_region.right(), 
                    img.atlas_region.top(), 
                    atlas_size.width - (first_empty_space.x + size.width),
                    first_empty_space.height
                ));
            }

            // space to the bottom
            if atlas_size.height - (first_empty_space.y + size.height) > 0 {
                empty_spaces.push(Rectangle::new(
                    img.atlas_region.left(), 
                    img.atlas_region.bottom(), 
                    first_empty_space.width,
                    atlas_size.height - (first_empty_space.y + size.height)
                ));
            }

            // sort empty spaces in descending order of size
            empty_spaces.sort_by(|a, b| {
                if a.width > b.width && a.height > b.height {
                    Ordering::Less
                } else {
                    Ordering::Greater
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
