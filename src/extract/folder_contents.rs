use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use anyhow::Result;

pub fn get_images_in_folder(operation_folder: &Path) -> Result<Vec<PathBuf>> {
    let mut images = Vec::<PathBuf>::new();

    let paths = read_dir(operation_folder)?;

    for path in paths {
        let path = path?.path();

        if path.is_dir() {
            let mut folder_images = get_images_in_folder(&path)?;
            images.append(&mut folder_images);
            continue;
        }

        let mut new_image = vec![path];
        images.append(&mut new_image);
    }

    Ok(images)
}
