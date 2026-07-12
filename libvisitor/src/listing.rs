use crate::VEntry;
use crate::VError;
use crate::entry::sort_rank;

use std::fs;
use std::path::Path;

pub fn list_dir(path: &Path) -> Result<Vec<VEntry>, VError> {
    let mut entries: Vec<VEntry> = Vec::new();
    let curr = fs::read_dir(path).unwrap();
    for path in curr {
        match path {
            Ok(entry) => {
                entries.push(VEntry::from_dir_entry(entry));
            } 
            Err(e) => {
                return Err(VError::FileReadError { e });
            }
        }
    }
    entries.sort_by_key(|e| (sort_rank(&e.kind), e.name.to_lowercase()));
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_list_empty() {
        let dir = tempdir().unwrap();
        let entries = list_dir(dir.path()).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_list_populated() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("hello.txt")).unwrap();

        let entries = list_dir(dir.path()).unwrap();
        assert_eq!(entries.len(), 1);
    }

}