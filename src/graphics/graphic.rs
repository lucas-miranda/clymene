use std::path::{
    Path,
    PathBuf
};

use crate::graphics::{
    animation::Animation,
    Image
};

pub enum Graphic {
    Empty,
    Image(Image),
    Animation(Animation)
}

impl Graphic {
    pub fn source_path(&self) -> Option<&PathBuf> {
        match self {
            Graphic::Image(img) => Some(&img.source_path),
            Graphic::Animation(anim) => Some(&anim.source_path),
            Graphic::Empty => None
        }
    }

    pub fn location(&self, source_root_directory: &Path) -> Option<PathBuf> {
        match self {
            Graphic::Image(img) => img.location(&source_root_directory),
            Graphic::Animation(anim) => anim.location(&source_root_directory),
            Graphic::Empty => None
        }
    }
}
