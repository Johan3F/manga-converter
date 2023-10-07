use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use printpdf::{image_crate, Image, ImageTransform, Mm, PdfDocument, PdfDocumentReference};

const SIZE_X: Mm = Mm(247.0);
const SIZE_Y: Mm = Mm(210.0);

pub fn create_pdf_file(file_path: &Path, images: Vec<PathBuf>) -> Result<()> {
    if file_path.file_stem().is_none() {
        bail!("the file_path doesn't reference a file: {:?}", file_path)
    }

    let file_name_stem = file_path.file_stem().unwrap();
    let file_name = file_name_stem.to_str();
    if file_name.is_none() {
        bail!("unable to get the original file name")
    }

    let (doc, _, _) = PdfDocument::new(file_name.unwrap(), SIZE_X, SIZE_Y, "Layer 1");

    populate_file(&doc, images)?;

    doc.save(&mut BufWriter::new(File::create(file_path).unwrap()))?;

    Ok(())
}

fn populate_file(doc: &PdfDocumentReference, images: Vec<PathBuf>) -> Result<()> {
    let mut index = 0;

    for image_path in images {
        if image_path.extension().is_none() || image_path.extension().unwrap().to_str().is_none() {
            println!("skipping image: {:?} -> Unable to get format", image_path);
            continue;
        }

        let image = get_image(&image_path)?;
        if image.is_none() {
            println!("skipping image: {:?} -> Format not supported", image_path);
        }
        let image = image.unwrap();

        let (page_index, layer_index) = doc.add_page(SIZE_X, SIZE_Y, "");
        let current_layer = doc.get_page(page_index).get_layer(layer_index);
        image.add_to_layer(current_layer.clone(), ImageTransform::default());

        index += 1;
        if index > 10 {
            break;
        }
    }

    Ok(())
}

fn get_image(image_path: &PathBuf) -> Result<Option<Image>> {
    let image_extension = image_path.extension().unwrap().to_str().unwrap();

    let image_file = File::open(image_path).unwrap();
    let image = match image_extension {
        "png" => Some(Image::try_from(image_crate::codecs::png::PngDecoder::new(
            image_file,
        )?)?),
        "jpg" => Some(Image::try_from(
            image_crate::codecs::jpeg::JpegDecoder::new(image_file)?,
        )?),
        _ => None,
    };

    Ok(image)
}
