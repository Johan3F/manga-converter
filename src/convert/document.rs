use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use printpdf::{Mm, PdfDocument, PdfDocumentReference, PdfLayerIndex, PdfPageIndex};

use crate::models::{ImageWrapper, DOUBLE_MANGA_WIDTH, DPI, MANGA_HEIGHT};

const MAX_SLOTS_PER_PAGE: usize = 2;

struct Page(Vec<ImageWrapper>);

pub struct Document {
    output_file: PathBuf,
    pages: Vec<Page>,
}

impl Document {
    pub fn new(title: PathBuf) -> Document {
        Document {
            output_file: title,
            pages: vec![],
        }
    }

    pub fn save(self, file_path: &Path) -> Result<()> {
        for page in self.pages {
            println!("{}", page.0.len());
        }

        // let (doc, initial_page, initial_layout) = PdfDocument::new(
        //     get_pdf_title_from_file_path(self.output_file)?,
        //     Mm(DOUBLE_MANGA_WIDTH),
        //     Mm(MANGA_HEIGHT),
        //     "",
        // );

        // doc.save(&mut BufWriter::new(File::create(file_path).unwrap()))?;
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
        self.pages
            .append(&mut vec![Page(vec![image]), Page(vec![])]);

        Ok(())
    }

    fn push_single_image(&mut self, image: ImageWrapper) -> Result<()> {
        let last_added_page = self.pages.last_mut();

        match last_added_page {
            None => self.pages.push(Page(vec![image])),
            Some(last_added_page) => match last_added_page.0.len() {
                0 => last_added_page.0.push(image),
                1 => last_added_page.0.push(image),
                _ => self.pages.push(Page(vec![image])),
            },
        };

        Ok(())
    }
}

fn create_doc_and_add_first_page(
    file_path: &Path,
    first_page: Page,
) -> Result<(PdfDocumentReference, PdfPageIndex, PdfLayerIndex)> {
    bail!("not implemented")
}

fn calculate_width(images: Vec<ImageWrapper>) -> Result<Mm> {
    // get the total width
    bail!("not implemented")
}

fn get_pdf_title_from_file_path(path: PathBuf) -> Result<String> {
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

// let mut available_images_slot = MAX_IMAGES_PER_PAGE;

// for image_path in images {
//     println!("Processing {:?}", image_path);
//     let image = get_image(&image_path)?;
//     if image.is_none() {
//         println!("skipping image: {:?} -> Format not supported", image_path);
//         continue;
//     }
//     let image = image.unwrap();

//     let scale_factor = image.get_scale_factor()?;

//     if available_images_slot == 0 || image.is_landscape() {
//         (current_page, current_layout) =
//             doc.add_page(Mm(DOUBLE_MANGA_WIDTH), Mm(MANGA_HEIGHT), "");
//         available_images_slot = MAX_IMAGES_PER_PAGE;
//     }
//     available_images_slot = match image.is_landscape() {
//         true => available_images_slot - 2,
//         false => available_images_slot - 1,
//     };
//     let current_layer = doc.get_page(current_page).get_layer(current_layout);

//     let transform = ImageTransform {
//         translate_x: Some(Mm(available_images_slot as f32 * MANGA_WIDTH)),
//         translate_y: None,
//         rotate: None,
//         scale_x: Some(scale_factor),
//         scale_y: Some(scale_factor),
//         dpi: Some(DPI),
//     };

//     image
//         .inner_image
//         .add_to_layer(current_layer.clone(), transform);
// }

// Ok(())
