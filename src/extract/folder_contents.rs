use std::{
    fs::{read_dir, DirEntry},
    io::Error,
    path::Path,
};

use crate::models::FolderEntry;

use anyhow::Result;

pub fn get_images_in_folder(operation_folder: &Path) -> Result<FolderEntry> {
    let mut images = Vec::<FolderEntry>::new();

    let raw_paths: Vec<Result<DirEntry, Error>> = read_dir(operation_folder)?.map(|r| r).collect();
    let mut paths: Vec<DirEntry> = raw_paths.into_iter().collect::<Result<Vec<_>, _>>()?;
    paths.sort_by_key(|element| element.path());

    for path in paths {
        let path = path.path();

        if path.is_dir() {
            let folder_images = get_images_in_folder(&path)?;
            images.append(&mut vec![folder_images]);
            continue;
        }

        images.append(&mut vec![FolderEntry::SingleEntry(path)]);
    }

    Ok(FolderEntry::Folder(images))
}
