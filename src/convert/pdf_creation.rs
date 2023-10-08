use std::{
    fs::File,
    path::{Path, PathBuf},
};

use super::{document::Document, image::ImageWrapper};
use crate::models::FolderEntry;

use anyhow::{bail, Result};
use printpdf::{image_crate, Image};

pub fn create_pdf_file(file_path: &Path, images: FolderEntry) -> Result<()> {
    if file_path.file_stem().is_none() {
        bail!("the file_path doesn't reference a file: {:?}", file_path)
    }

    let file_name_stem = file_path.file_stem().unwrap();
    let file_name = file_name_stem.to_str();
    if file_name.is_none() {
        bail!("unable to get the original file name")
    }

    let document = Document::new(file_name.unwrap());

    process_entry(&document, &images)?;

    document.save(file_path)?;

    Ok(())
}

fn process_entry(document: &Document, entry: &FolderEntry) -> Result<()> {
    match entry {
        FolderEntry::SingleEntry(entry) => process_single_file_entry(&document, entry),
        FolderEntry::Folder(entries) => process_folder_entry(&document, entries),
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

fn process_single_file_entry(document: &Document, image: &PathBuf) -> Result<()> {
    println!("process_single_file_entry");
    Ok(())
}

fn process_folder_entry(document: &Document, entries: &Vec<FolderEntry>) -> Result<()> {
    println!("process_folder_entry");
    Ok(())
}
