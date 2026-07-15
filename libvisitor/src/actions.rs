use crate::{VEntry, VError};

use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub fn act_create_file(path: PathBuf) -> Result<File, VError>{
    match File::open(path) {
        Err(e) => { return Err(VError::FileCreateError { e }) },
        Ok(f) => { return Ok(f) }
    }
}

pub fn act_create_dir(path: PathBuf) -> Result<(), VError> {
    match fs::create_dir(path) {
        Err(e) => { return Err(VError::DirCreateError { e }) },
        _ => Ok(())
    }
}

pub fn act_delete(path: PathBuf) -> Result<(), VError> {
    match fs::remove_file(path) {
        Err(e) => { return Err(VError::DirCreateError { e }) },
        _ => Ok(())
    }
}

pub fn act_copy(from: PathBuf, to: PathBuf) -> Result<u64, VError> {
    match fs::copy(from, to) {
        Ok(size) => {
            Ok(size)
        },
        Err(e) => {
            Err(VError::FileCopyError { e })
        }
    }
}

pub fn act_move(from: PathBuf, to: PathBuf) -> Result<u64, VError> {
    act_copy(from, to)
    // remove later
}

pub fn act_rename() {
    //fs::rename(from, to)
}

pub fn act_create_symlink() {
    //
}

pub fn act_compress() {
    //
}

pub fn act_extract() {
    //
}   

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_create_dir() {
        let dir = tempdir().unwrap();
        let new_path = dir.path().join("example");
        act_create_dir(new_path.clone()).unwrap();
        assert!(new_path.exists());
    }

}