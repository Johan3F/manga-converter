mod names;
mod pdf_creation;

use std::path::{Path, PathBuf};

use anyhow::Result;

pub fn convert_to_pdf(
    original_file_path: &Path,
    destination_folder: &Path,
    images: Vec<PathBuf>,
) -> Result<()> {
    let destination_file_path =
        names::get_output_file_name(original_file_path, destination_folder)?;

    println!("storing resulting file in {:?}", destination_file_path);

    pdf_creation::create_pdf_file(&destination_file_path, images)?;

    Ok(())
}
