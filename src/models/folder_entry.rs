use crate::models::ImageWrapper;

pub enum FolderEntry {
    SingleEntry(ImageWrapper),
    Folder(Vec<FolderEntry>),
}
