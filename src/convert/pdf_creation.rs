use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use super::constants::{DOUBLE_MANGA_WIDTH, DPI, MANGA_HEIGHT, MANGA_WIDTH};

use super::image::ImageWrapper;
use anyhow::{bail, Result};
use printpdf::{
    image_crate, Image, ImageTransform, Mm, PdfDocument, PdfDocumentReference, PdfLayerIndex,
    PdfPageIndex,
};

const MAX_IMAGES_PER_PAGE: u32 = 2;

pub fn create_pdf_file(file_path: &Path, images: Vec<PathBuf>) -> Result<()> {
    if file_path.file_stem().is_none() {
        bail!("the file_path doesn't reference a file: {:?}", file_path)
    }

    let file_name_stem = file_path.file_stem().unwrap();
    let file_name = file_name_stem.to_str();
    if file_name.is_none() {
        bail!("unable to get the original file name")
    }

    let (doc, initial_page, initial_layout) = PdfDocument::new(
        file_name.unwrap(),
        Mm(DOUBLE_MANGA_WIDTH),
        Mm(MANGA_HEIGHT),
        "",
    );
    populate_file(&doc, initial_page, initial_layout, images)?;

    doc.save(&mut BufWriter::new(File::create(file_path).unwrap()))?;

    Ok(())
}

fn populate_file(
    doc: &PdfDocumentReference,
    mut current_page: PdfPageIndex,
    mut current_layout: PdfLayerIndex,
    images: Vec<PathBuf>,
) -> Result<()> {
    let mut available_images_slot = MAX_IMAGES_PER_PAGE;

    for image_path in images {
        println!("Processing {:?}", image_path);
        let image = get_image(&image_path)?;
        if image.is_none() {
            println!("skipping image: {:?} -> Format not supported", image_path);
            continue;
        }
        let image = image.unwrap();

        let scale_factor = image.get_scale_factor()?;

        if available_images_slot == 0 || image.is_landscape() {
            (current_page, current_layout) =
                doc.add_page(Mm(DOUBLE_MANGA_WIDTH), Mm(MANGA_HEIGHT), "");
            available_images_slot = MAX_IMAGES_PER_PAGE;
        }
        available_images_slot = match image.is_landscape() {
            true => available_images_slot - 2,
            false => available_images_slot - 1,
        };
        let current_layer = doc.get_page(current_page).get_layer(current_layout);

        let transform = ImageTransform {
            translate_x: Some(Mm(available_images_slot as f32 * MANGA_WIDTH)),
            translate_y: None,
            rotate: None,
            scale_x: Some(scale_factor),
            scale_y: Some(scale_factor),
            dpi: Some(DPI),
        };

        image
            .inner_image
            .add_to_layer(current_layer.clone(), transform);
    }

    Ok(())
}

fn get_image(image_path: &PathBuf) -> Result<Option<ImageWrapper>> {
    if image_path.extension().is_none() || image_path.extension().unwrap().to_str().is_none() {
        return Ok(None);
    }

    let image_extension = image_path.extension().unwrap().to_str().unwrap();

    let image_file = File::open(image_path).unwrap();
    let image = match image_extension {
        "png" => Some(ImageWrapper::new(Image::try_from(
            image_crate::codecs::png::PngDecoder::new(image_file)?,
        )?)),
        "jpg" => Some(ImageWrapper::new(Image::try_from(
            image_crate::codecs::jpeg::JpegDecoder::new(image_file)?,
        )?)),
        _ => None,
    };

    Ok(image)
}
