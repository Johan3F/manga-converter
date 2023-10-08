mod convert;
mod extract;
mod models;

use std::{
    fs::{create_dir_all, remove_dir_all},
    path::Path,
};

use anyhow::{bail, Result};

fn main() {
    let file_paths = vec![
        // Path::new("local/Gintama, v01 [2004] [Viz] [senfgurke2].cbz"),
        Path::new("local/Gintama, v02 [2004] [Viz] [senfgurke2].cbz"),
        // Path::new("local/Gintama, v03 [2004] [Viz] [senfgurke2].cbz"),
        // Path::new("local/Gintama, v04 [2004] [Viz] [senfgurke2].cbz"),
    ];
    let operation_folder = Path::new("local/extracted");
    let destination_folder = Path::new("local/converted");

    for file_path in file_paths {
        ensure_destination_and_operation_exist(destination_folder, operation_folder)
            .expect("unable to ensure the destination and operation folders");

        let images = extract::extract(file_path, operation_folder)
            .expect("unable to extract images from file");

        convert::convert_to_pdf(file_path, destination_folder, images)
            .expect("unable to convert to pdf");
    }
}

fn ensure_destination_and_operation_exist(
    destination_folder: &Path,
    operation_folder: &Path,
) -> Result<()> {
    let ensure_folder_exist = |folder: &Path| -> Result<()> {
        if !folder.exists() {
            create_dir_all(folder)?;
        }

        if !folder.is_dir() {
            bail!("the destination folder is not a folder");
        }

        Ok(())
    };

    ensure_folder_exist(destination_folder)?;
    ensure_folder_exist(operation_folder)?;

    remove_dir_all(operation_folder)?;

    Ok(())
}
