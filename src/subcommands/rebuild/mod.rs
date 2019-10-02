use crate::configuration::Configuration;
use sha2::{Digest, Sha512};
use std::fs::{read_dir, DirEntry, File};
use std::intrinsics::copy;
use std::io;
use std::path::Path;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn handle_file(dir_entry: &DirEntry) {
    let mut file = File::open(&dir_entry.path()).unwrap();
    let mut hasher = Sha512::new();

    let n = io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.result();

    println!("{} -> {:x}", dir_entry.path().to_str().unwrap(), hash);
}

pub fn run_subcommand(config: &Configuration) {
    let directories = config.observed_directories.clone();
    for directory in directories {
        visit_dirs(Path::new(&directory), &handle_file);
    }
}
