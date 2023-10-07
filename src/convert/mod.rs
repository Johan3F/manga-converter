use std::{fs::read_dir, path::Path};

use anyhow::{anyhow, Result};

pub fn convert_to_pdf(operation_folder: &Path, destination_folder: &Path) -> Result<()> {
    let images = get_images_in_folder(operation_folder)?;
    Ok(())
}

fn get_images_in_folder(operation_folder: &Path) -> Result<Vec<&Path>> {
    let paths = read_dir(operation_folder)?;

    for path in paths {
        println!("found file: {:?}", path);
    }
    Ok(vec![])
}
