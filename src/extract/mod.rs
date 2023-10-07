mod folder_contents;
mod uncompress;

use std::path::{Path, PathBuf};

use anyhow::Result;

pub fn extract(file_path: &Path, operation_folder: &Path) -> Result<Vec<PathBuf>> {
    uncompress::uncompress_file(file_path, operation_folder)?;

    folder_contents::get_images_in_folder(operation_folder)
}
