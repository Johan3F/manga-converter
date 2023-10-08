use std::{fs::File, io::BufWriter, path::Path};

use anyhow::Result;
use printpdf::{Mm, PdfDocument, PdfDocumentReference, PdfLayerIndex, PdfPageIndex};

use super::constants::{DOUBLE_MANGA_WIDTH, MANGA_HEIGHT};

pub struct Document {
    file: PdfDocumentReference,
    current_page: PdfPageIndex,
    current_layer: PdfLayerIndex,
}

impl Document {
    pub fn new(title: &str) -> Document {
        let (doc, initial_page, initial_layout) =
            PdfDocument::new(title, Mm(DOUBLE_MANGA_WIDTH), Mm(MANGA_HEIGHT), "");

        Document {
            file: doc,
            current_page: initial_page,
            current_layer: initial_layout,
        }
    }

    pub fn save(self, file_path: &Path) -> Result<()> {
        self.file
            .save(&mut BufWriter::new(File::create(file_path).unwrap()))?;
        Ok(())
    }
}
