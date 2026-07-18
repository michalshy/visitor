use crate::VError;

use std::fs::{self, File};
use std::path::PathBuf;

pub fn act_create_file(path: PathBuf) -> Result<File, VError>{
    match File::open(path) {
        Err(e) => { return Err(VError::Create { e }) },
        Ok(f) => { return Ok(f) }
    }
}

pub fn act_create_dir(path: PathBuf) -> Result<(), VError> {
    match fs::create_dir(path) {
        Err(e) => { return Err(VError::Create { e }) },
        _ => Ok(())
    }
}

pub fn act_delete(path: PathBuf) -> Result<(), VError> {
    match fs::remove_file(path) {
        Err(e) => { return Err(VError::Delete { e }) },
        _ => Ok(())
    }
}

pub fn act_copy(from: PathBuf, to: PathBuf) -> Result<u64, VError> {
    match fs::copy(from, to) {
        Ok(size) => {
            Ok(size)
        },
        Err(e) => {
            Err(VError::Copy { e })
        }
    }
}

pub fn act_move(from: PathBuf, to: PathBuf) -> Result<(), VError> {
    if let Some(from_p) = from.clone().parent() 
        && let Some(to_p) = to.clone().parent() {
        if from_p == to_p {
            return act_rename(from, to);
        }
    } 

    act_copy(from.clone(), to)?;
    act_delete(from)
}

pub fn act_rename(from: PathBuf, to: PathBuf) -> Result<(), VError>  {
    match fs::rename(from, to) {
        Err(e) => { return Err(VError::Rename { e }) },
        _ => Ok(())
    }
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
    use tempfile::tempdir;

    #[test]
    fn test_create_dir() {
        let dir = tempdir().unwrap();
        let new_path = dir.path().join("example");
        act_create_dir(new_path.clone()).unwrap();
        assert!(new_path.exists());
    }

}