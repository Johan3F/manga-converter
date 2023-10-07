use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use printpdf::{Mm, PdfDocument, PdfDocumentReference};

pub fn create_pdf_file(file_path: &Path, images: Vec<PathBuf>) -> Result<()> {
    if file_path.file_stem().is_none() {
        bail!("the file_path doesn't reference a file: {:?}", file_path)
    }

    let file_name_stem = file_path.file_stem().unwrap();
    let file_name = file_name_stem.to_str();
    if file_name.is_none() {
        bail!("unable to get the original file name")
    }

    let (doc, _, _) = PdfDocument::new(file_name.unwrap(), Mm(247.0), Mm(210.0), "Layer 1");

    populate_file(&doc, images)?;

    doc.save(&mut BufWriter::new(File::create(file_path).unwrap()))?;

    Ok(())
}

fn populate_file(_doc: &PdfDocumentReference, _images: Vec<PathBuf>) -> Result<()> {
    Ok(())
}
