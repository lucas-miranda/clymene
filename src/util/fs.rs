use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
};

pub fn is_dir_empty<P: AsRef<Path>>(dir: P) -> io::Result<bool> {
    if fs::read_dir(dir)?.next().is_some() {
        return Ok(false);
    }

    Ok(true)
}

pub fn for_every_file<P: AsRef<Path>, F: FnMut(&DirEntry)>(
    dir: P,
    callback: &mut F,
) -> io::Result<()> {
    for dir_entry in fs::read_dir(dir)? {
        let entry = dir_entry?;
        let path = entry.path();
        if path.is_dir() {
            match for_every_file(&path, callback) {
                Ok(()) => (),
                Err(e) => return Err(e),
            }
        } else {
            callback(&entry);
        }
    }

    Ok(())
}

pub fn find<P: AsRef<Path>, F: FnMut(&DirEntry) -> bool>(
    dir: P,
    filter: &mut F,
) -> io::Result<Option<DirEntry>> {
    for dir_entry in fs::read_dir(dir)? {
        let entry = dir_entry?;
        let path = entry.path();

        if filter(&entry) {
            return Ok(Some(entry));
        }

        if !path.is_dir() {
            continue;
        }

        match find(&path, filter) {
            Ok(entry) => {
                if let Some(found_entry) = entry {
                    return Ok(Some(found_entry));
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(None)
}
