pub mod encoding;
pub mod fs;

/*
pub mod util {
    use std::{
        fs::{ self, DirEntry },
        path::{ Path, PathBuf },
        str::{ FromStr },
        io::{ self }
    };

    pub fn list_files<'a>(path: &'a str) -> Vec<PathBuf> {
        let mut files: Vec<PathBuf> = Vec::new();

        let path_buf = PathBuf::from_str(path).unwrap();
        for_every_file(
            path_buf.as_path(),
            &mut |entry: &DirEntry| {
                let path_buf = entry.path();
                let path_data = path_buf.as_path().to_owned();
                //let clone_path = String::from(path.to_str().unwrap());
                //println!("{}", path_data.);
                files.push(path_data);
            }
        ).unwrap();

        return files;
    }

    pub fn stored_folder<'a>(pathbuf: &PathBuf, root_folder: &'a str) -> Option<PathBuf> {
        let mut total_path;
        let mut current_path = pathbuf.as_path();
        let file_type = pathbuf.metadata().unwrap().file_type();

        if file_type.is_dir() {
            total_path = PathBuf::from(pathbuf.file_name().unwrap());
        } else {
            total_path = PathBuf::new();
            current_path = current_path.parent().unwrap(); // pathbuf points to a file
        }

        let mut current_filename = current_path.file_name().unwrap().to_str().unwrap();

        while current_filename != root_folder {
            total_path = Path::new(current_filename).join(total_path);

            current_path = match current_path.parent() {
                Some(parent_path) => parent_path,
                None => return None
            };

            current_filename = match current_path.file_name() {
                Some(filename) => filename.to_str().unwrap(),
                None => return None
            };
        }

        Some(total_path)
    }

    pub fn pathname_normalize<'a>(path: &'a str) -> String {
        let path_normalized;

        if cfg!(windows) {
            path_normalized = path.replace("\\", "/");
        } else {
            path_normalized = String::from(path);
        }

        path_normalized
    }

    pub fn pathbuf_normalize(path: &PathBuf) -> String {
        pathname_normalize(path.to_str().unwrap())
    }

    pub fn plural_special<'a>(count: &usize, plural: &'a str) -> String {
        if *count == 1 {
            return String::from("");
        }

        return String::from(plural);
    }

    pub fn plural<'a>(count: &usize) -> String {
        plural_special(&count, "s")
    }
}
*/
