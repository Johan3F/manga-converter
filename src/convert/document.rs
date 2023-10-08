use std::{collections::VecDeque, fs::File, io::BufWriter, path::Path};

use anyhow::{bail, Result};
use printpdf::{
    ImageTransform, Mm, PdfDocument, PdfDocumentReference, PdfLayerIndex, PdfPageIndex,
};

use crate::models::{ImageWrapper, DPI, MANGA_HEIGHT};

const MAX_SLOTS_PER_PAGE: usize = 2;

struct Page(Vec<ImageWrapper>);

pub struct Document {
    pages: VecDeque<Page>,
}

impl Document {
    pub fn new() -> Document {
        Document {
            pages: VecDeque::new(),
        }
    }

    pub fn save(mut self, file_path: &Path) -> Result<()> {
        let first_page = self.pages.pop_front();
        if first_page.is_none() {
            return Ok(());
        }
        let first_page = first_page.unwrap();

        let doc = create_doc_and_add_first_page(file_path, first_page)?;

        for page in self.pages {
            add_page(&doc, page)?;
        }
        doc.save(&mut BufWriter::new(File::create(file_path).unwrap()))?;
        Ok(())
    }

    pub fn push_image(&mut self, image: ImageWrapper) -> Result<()> {
        if image.is_landscape() {
            self.push_double_image(image)?;
            return Ok(());
        }

        self.push_single_image(image)?;
        Ok(())
    }

    fn push_double_image(&mut self, image: ImageWrapper) -> Result<()> {
        self.pages.push_back(Page(vec![image]));
        self.pages.push_back(Page(vec![]));

        Ok(())
    }

    fn push_single_image(&mut self, image: ImageWrapper) -> Result<()> {
        let last_added_page = self.pages.back_mut();
        if last_added_page.is_none() {
            self.pages.push_back(Page(vec![image]));
            return Ok(());
        }
        let last_added_page = last_added_page.unwrap();

        if last_added_page.0.len() >= MAX_SLOTS_PER_PAGE {
            self.pages.push_back(Page(vec![image]));
            return Ok(());
        }

        last_added_page.0.push(image);

        Ok(())
    }
}

fn create_doc_and_add_first_page(
    file_path: &Path,
    first_page: Page,
) -> Result<PdfDocumentReference> {
    let width_in_mm = calculate_width_in_page(&first_page);

    let (doc, page_index, layout_index) = PdfDocument::new(
        get_pdf_title_from_file_path(file_path)?,
        width_in_mm,
        Mm(MANGA_HEIGHT),
        "",
    );

    add_images_to_page(first_page, &doc, &page_index, &layout_index);

    Ok(doc)
}

fn add_page(doc: &PdfDocumentReference, page: Page) -> Result<()> {
    let width_in_mm = calculate_width_in_page(&page);

    let (current_page, current_layout) = doc.add_page(width_in_mm, Mm(MANGA_HEIGHT), "");

    add_images_to_page(page, &doc, &current_page, &current_layout);
    Ok(())
}

fn add_images_to_page(
    page: Page,
    doc: &PdfDocumentReference,
    page_index: &PdfPageIndex,
    layout_index: &PdfLayerIndex,
) {
    let current_layer = doc
        .get_page(page_index.to_owned())
        .get_layer(layout_index.to_owned());

    let total_width_in_mm = calculate_width_in_page(&page);
    let mut width_accumulative_offset = 0.0;
    for image in page.0 {
        let transform = ImageTransform {
            translate_x: Some(Mm(total_width_in_mm.0
                - image.get_scaled_width_in_mm()
                - width_accumulative_offset)),
            translate_y: None,
            rotate: None,
            scale_x: Some(image.scale_factor_to_manga_heigth),
            scale_y: Some(image.scale_factor_to_manga_heigth),
            dpi: Some(DPI),
        };

        width_accumulative_offset += image.get_scaled_width_in_mm();

        image
            .inner_image
            .add_to_layer(current_layer.clone(), transform);
    }
}

fn calculate_width_in_page(page: &Page) -> Mm {
    Mm(page
        .0
        .iter()
        .fold(0.0, |acc, image| acc + image.get_scaled_width_in_mm()))
}

fn get_pdf_title_from_file_path(path: &Path) -> Result<String> {
    if path.file_stem().is_none() {
        bail!("the file_path doesn't reference a file: {:?}", path)
    }

    let file_name_stem = path.file_stem().unwrap();
    let file_name = file_name_stem.to_str();
    if file_name.is_none() {
        bail!("unable to get the original file name")
    }

    Ok(file_name.unwrap().to_owned())
}
