use std::{fs::File, path::Path};

use anyhow::{bail, Result};
use zip::ZipArchive;

pub fn uncompress_file(file_path: &Path, destination_folder: &Path) -> Result<()> {
    verify_file(file_path)?;

    let file = File::open(file_path)?;

    let mut archive = ZipArchive::new(file).unwrap();
    archive.extract(destination_folder)?;

    return Ok(());
}

fn verify_file(file_path: &Path) -> Result<()> {
    let expected_formats = vec!["cbz"];

    if file_path.extension().is_none() {
        bail!("the file should have an extension");
    }

    let extension = file_path.extension().unwrap();
    if extension.to_str().is_none() {
        bail!("the file extension is not recognized");
    };

    let extension = extension.to_str().unwrap();
    if !expected_formats.contains(&extension) {
        bail!("the file extension is not supported");
    }

    Ok(())
}
