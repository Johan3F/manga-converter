use std::{
    fs::{create_dir_all, remove_dir_all, File},
    path::Path,
};

use anyhow::{anyhow, Result};
use zip::ZipArchive;

pub fn extract_into_folder(file_path: &Path, destination_folder: &Path) -> Result<()> {
    verify_file(file_path)?;
    ensure_empty_destination_folder(destination_folder)?;

    println!("file_path: {:?}", file_path);
    println!("folder_path: {:?}", destination_folder);

    let file = File::open(file_path)?;

    let mut archive = ZipArchive::new(file).unwrap();
    archive.extract(destination_folder)?;

    return Ok(());
}

fn verify_file(file_path: &Path) -> Result<()> {
    let expected_formats = vec!["cbz"];

    if file_path.extension().is_none() {
        return Err(anyhow!("the file should have an extension"));
    }

    let extension = file_path.extension().unwrap();
    if extension.to_str().is_none() {
        return Err(anyhow!("the file extension is not recognized"));
    };

    let extension = extension.to_str().unwrap();
    if !expected_formats.contains(&extension) {
        return Err(anyhow!("the file extension is not supported"));
    }

    Ok(())
}

fn ensure_empty_destination_folder(destination_folder: &Path) -> Result<()> {
    if !destination_folder.exists() {
        create_dir_all(destination_folder)?;
    }

    if !destination_folder.is_dir() {
        return Err(anyhow!("the destination folder is not a folder"));
    }

    remove_dir_all(destination_folder)?;

    Ok(())
}
