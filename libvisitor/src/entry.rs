use std::{fs::{self, DirEntry}, os::unix::fs::MetadataExt, path::PathBuf, time::SystemTime};

pub struct VEntry
{
    name: String,
    path: String,
    kind: VKind,
    size: u64,
    modified: SystemTime,
    permissions: VPermissions,
    hidden: bool,
}

impl VEntry {
    pub fn from_dir_entry(de: DirEntry) -> VEntry {
        let name = de.file_name().into_string().unwrap();
        let path = de.path().to_string_lossy().to_string();
        let metadata = de.metadata().unwrap();
        let size = metadata.size();
        let modified = metadata.modified().unwrap();
        let permissions = VPermissions::from_metadata(&metadata);
        let hidden = is_hidden(&name, &metadata);

        let file_type = de.file_type().unwrap();
        let mut kind = VKind::File;
        if file_type.is_dir() {
            kind = VKind::Dir;
        }
        else if file_type.is_symlink() {
            let broken = !de.path().exists();
            let symlink = fs::read_link(de.path());
            match symlink {
                Ok(target) => {
                    kind = VKind::Symlink { 
                        target: Some(target), 
                        broken 
                    }
                }
                Err(_) => {
                    kind = VKind::Symlink { target: None, broken }
                }
            }            
        }

        VEntry { 
            name, 
            path, 
            kind,
            size, 
            modified, 
            permissions, 
            hidden
        }
    }
}

pub enum VKind
{
    File,
    Dir,
    Symlink { target: Option<PathBuf>, broken: bool }
}

pub enum VPermissions {
    Unix { mode: u32 },
    Basic { readonly: bool },
}

impl VPermissions {
    #[cfg(unix)]
    fn from_metadata(metadata: &std::fs::Metadata) -> VPermissions {
        use std::os::unix::fs::PermissionsExt;
        VPermissions::Unix { mode: metadata.permissions().mode() }
    }

    #[cfg(windows)]
    fn from_metadata(metadata: &std::fs::Metadata) -> VPermissions {
        VPermissions::Basic { readonly: metadata.permissions().readonly() }
    }
}

impl std::fmt::Display for VPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VPermissions::Unix { mode } => {
                let bits = [
                    (0o400, 'r'), (0o200, 'w'), (0o100, 'x'),
                    (0o040, 'r'), (0o020, 'w'), (0o010, 'x'),
                    (0o004, 'r'), (0o002, 'w'), (0o001, 'x'),
                ];
                let s: String = bits.iter()
                    .map(|(mask, ch)| if mode & mask != 0 { *ch } else { '-' })
                    .collect();
                write!(f, "{s}")
            }
            VPermissions::Basic { readonly } => {
                write!(f, "{}", if *readonly { "r--r--r--" } else { "rw-rw-rw-" })
            }
        }
    }
}

#[cfg(unix)]
fn is_hidden(name: &str, _metadata: &std::fs::Metadata) -> bool {
    name.starts_with('.')
}

#[cfg(windows)]
fn is_hidden(_name: &str, metadata: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
    metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0
}