use std::{
    fs::{read_dir, DirEntry},
    io::Error,
    path::{Path, PathBuf},
};

use anyhow::Result;

pub fn get_images_in_folder(operation_folder: &Path) -> Result<Vec<PathBuf>> {
    let mut images = Vec::<PathBuf>::new();

    let raw_paths: Vec<Result<DirEntry, Error>> = read_dir(operation_folder)?.map(|r| r).collect();
    let mut paths: Vec<DirEntry> = raw_paths.into_iter().collect::<Result<Vec<_>, _>>()?;
    paths.sort_by_key(|element| element.path());

    for path in paths {
        let path = path.path();

        if path.is_dir() {
            let mut folder_images = get_images_in_folder(&path)?;
            folder_images.sort();
            images.append(&mut folder_images);
            continue;
        }

        let mut new_image = vec![path];
        images.append(&mut new_image);
    }

    Ok(images)
}
