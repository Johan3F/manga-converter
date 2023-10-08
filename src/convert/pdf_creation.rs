use std::path::Path;

use super::document::Document;
use crate::models::{FolderEntry, ImageWrapper};

use anyhow::Result;

pub fn create_pdf_file(file_path: &Path, images: FolderEntry) -> Result<()> {
    let mut document = Document::new(file_path.to_path_buf());

    process_entry(&mut document, images)?;

    document.save(file_path)?;

    Ok(())
}

fn process_entry(mut document: &mut Document, entry: FolderEntry) -> Result<()> {
    match entry {
        FolderEntry::SingleEntry(entry) => process_single_file_entry(&mut document, entry),
        FolderEntry::Folder(entries) => process_folder_entry(&mut document, entries),
    }
}

fn process_single_file_entry(document: &mut Document, image: ImageWrapper) -> Result<()> {
    println!("process_single_file_entry");
    document.push_image(image)?;

    Ok(())
}

fn process_folder_entry(document: &mut Document, entries: Vec<FolderEntry>) -> Result<()> {
    println!("process_folder_entry");
    for entry in entries {
        process_entry(document, entry)?;
    }
    Ok(())
}
