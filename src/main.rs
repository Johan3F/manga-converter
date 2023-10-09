mod convert;
mod extract;
mod models;

use std::{fs::create_dir_all, path::Path};

use anyhow::{bail, Result};

fn main() {
    let file_paths = vec![
        Path::new("local/Gintama, v01 [2004] [Viz] [senfgurke2].cbz"),
        Path::new("local/Gintama, v02 [2004] [Viz] [senfgurke2].cbz"),
        Path::new("local/Gintama, v03 [2004] [Viz] [senfgurke2].cbz"),
        Path::new("local/Gintama, v04 [2004] [Viz] [senfgurke2].cbz"),
    ];
    let destination_folder = Path::new("local/converted");
    ensure_destination(destination_folder)
        .expect("unable to ensure that the destination folder exists");

    for file_path in file_paths {
        process(file_path, destination_folder).expect("unable to process file");
    }
}

fn process(file_path: &Path, destination_folder: &Path) -> Result<()> {
    println!("Processing: {:?}", file_path);
    let operation_folder = tempfile::tempdir()?;

    let images = extract::extract(file_path, operation_folder.path())?;
    convert::convert_to_pdf(file_path, destination_folder, images)?;

    Ok(())
}

fn ensure_destination(destination_folder: &Path) -> Result<()> {
    if !destination_folder.exists() {
        create_dir_all(destination_folder)?;
    }

    if !destination_folder.is_dir() {
        bail!("the destination folder is not a folder");
    }

    Ok(())
}
