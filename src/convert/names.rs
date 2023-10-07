use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

pub fn get_output_file_name(
    original_file_path: &Path,
    destination_folder: &Path,
) -> Result<PathBuf> {
    if original_file_path.file_name().is_none() {
        bail!(
            "original file name is not found in: {:?}",
            original_file_path
        );
    }
    if !destination_folder.is_dir() {
        bail!(
            "the destination folder is not a folder: {:?}",
            destination_folder
        );
    }

    let original_file_name = original_file_path.file_name().unwrap();
    let destination_file_name = Path::new(original_file_name).with_extension("pdf");
    let destination_path = destination_folder.join(destination_file_name);

    Ok(destination_path)
}
