use std::{path::PathBuf, time::SystemTime};

pub struct VEntry
{
    name: String,
    path: String,
    kind: VKind,
    size: u64,
    modified: SystemTime,
    permissions: Permissions,
    hidden: bool,
}

pub enum VKind
{
    File,
    Dir,
    Symlink { target: Option<PathBuf>, broken: bool }
}

pub struct Permissions
{
    r: bool,
    w: bool,
    e: bool,
}