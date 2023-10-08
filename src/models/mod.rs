use std::{
    fmt::{Debug, Formatter, Result},
    path::PathBuf,
};

pub enum FolderEntry {
    SingleEntry(PathBuf),
    Folder(Vec<FolderEntry>),
}

impl Debug for FolderEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::SingleEntry(entry) => f
                .debug_struct("FolderEntry::SingleEntry")
                .field("", entry)
                .finish(),
            Self::Folder(entry) => f.debug_list().entries(entry.iter()).finish(),
        }
    }
}
