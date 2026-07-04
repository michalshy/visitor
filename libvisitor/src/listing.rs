use crate::VEntry;
use crate::VError;

use std::fs;
use std::env;

pub fn list_dir() -> Result<Vec<VEntry>, VError> {
    let mut entries: Vec<VEntry> = Vec::new();
    let curr = fs::read_dir(env::current_dir().unwrap()).unwrap();
    for path in curr {
        match path {
            Ok(entry) => {
                entries.push(VEntry::from_dir_entry(entry));
            } 
            Err(e) => {
                return Err(VError::FILE_READ_ERROR{e: e.into()});
            }
        }
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_entry_metadata() {

    }

}