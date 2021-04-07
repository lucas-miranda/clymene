use std::{
    io::ErrorKind,
    path::PathBuf,
    fs,
    thread,
    time
};

pub fn clear_cache(cache_path: &PathBuf) {
    println!("> Clearing cache files");

    // remove cache dir and all it's contents
    if cache_path.exists() {
        fs::remove_dir_all(cache_path.as_path()).unwrap();
    }

    // recreate cache dir
    let mut permission_denied_retries = 5;

    'create_dir: loop {
        match fs::create_dir(cache_path.as_path()) {
            Ok(()) => break 'create_dir,
            Err(e) => {
                match e.kind() {
                    ErrorKind::PermissionDenied => {
                        // probably 'fs::remove_dir_all' still holds folder lock
                        // so, just wait a bit and retry

                        if permission_denied_retries == 0 {
                            // panic, we reached our max tries to recreate cache dir!
                            panic!(e);
                        }

                        let duration = time::Duration::from_millis(100u64);
                        thread::sleep(duration);

                        permission_denied_retries -= 1;
                    },
                    ErrorKind::AlreadyExists => {
                        // ignore dir creating process
                        // our job is already done by someone else
                        break 'create_dir;
                    },
                    _ => {
                        println!("Error kind: {:?}", e.kind());
                        panic!(e);
                    }
                };

                ()
            }
        };
    }

    println!("|- Cache files has been cleared");
}
