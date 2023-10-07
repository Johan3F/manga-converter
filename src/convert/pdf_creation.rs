use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use printpdf::{image_crate, Image, ImageTransform, Mm, PdfDocument, PdfDocumentReference};

const MANGA_WIDTH: f32 = 127.0;
const DOUBLE_MANGA_WIDTH: f32 = 127.0 * 2.0;
const MANGA_HEIGHT: f32 = 190.5;

const DPI: f32 = 300.0;

const MM_PER_INCH: f32 = 25.4;
const MM_PER_DPI: f32 = MM_PER_INCH / DPI;

pub fn create_pdf_file(file_path: &Path, images: Vec<PathBuf>) -> Result<()> {
    if file_path.file_stem().is_none() {
        bail!("the file_path doesn't reference a file: {:?}", file_path)
    }

    let file_name_stem = file_path.file_stem().unwrap();
    let file_name = file_name_stem.to_str();
    if file_name.is_none() {
        bail!("unable to get the original file name")
    }

    let (doc, _, _) = PdfDocument::new(
        file_name.unwrap(),
        Mm(DOUBLE_MANGA_WIDTH),
        Mm(MANGA_HEIGHT),
        "",
    );
    populate_file(&doc, images)?;

    doc.save(&mut BufWriter::new(File::create(file_path).unwrap()))?;

    Ok(())
}

fn populate_file(doc: &PdfDocumentReference, images: Vec<PathBuf>) -> Result<()> {
    let mut index = 0;

    for image_path in images {
        println!("Processing {:?}", image_path);
        let image = get_image(&image_path)?;
        if image.is_none() {
            println!("skipping image: {:?} -> Format not supported", image_path);
            continue;
        }
        let image = image.unwrap();

        let scale_factor = get_correct_scale_factor(&image)?;

        let (page_index, layer_index) = doc.add_page(Mm(DOUBLE_MANGA_WIDTH), Mm(MANGA_HEIGHT), "");
        let current_layer = doc.get_page(page_index).get_layer(layer_index);

        let transform = ImageTransform {
            translate_x: None,
            translate_y: None,
            rotate: None,
            scale_x: Some(scale_factor),
            scale_y: Some(scale_factor),
            dpi: Some(DPI),
        };

        image.add_to_layer(current_layer.clone(), transform);

        index += 1;
        if index > 10 {
            break;
        }
    }

    Ok(())
}

fn get_image(image_path: &PathBuf) -> Result<Option<Image>> {
    if image_path.extension().is_none() || image_path.extension().unwrap().to_str().is_none() {
        return Ok(None);
    }

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

fn get_correct_scale_factor(image: &Image) -> Result<f32> {
    let mut is_double = false;
    let width_in_mm = image.image.width.0 as f32 * MM_PER_DPI;
    let height_in_mm = image.image.height.0 as f32 * MM_PER_DPI;

    let width_scale_factor = match height_in_mm > width_in_mm {
        true => MANGA_WIDTH / width_in_mm,
        false => {
            is_double = true;
            DOUBLE_MANGA_WIDTH / width_in_mm
        }
    };

    let mut scale_factor = width_scale_factor;
    if !is_in_manga_bounds_after_scale(width_in_mm, height_in_mm, width_scale_factor, is_double) {
        scale_factor = MANGA_HEIGHT / height_in_mm;
    }

    return Ok(scale_factor);
}

fn is_in_manga_bounds_after_scale(
    width: f32,
    height: f32,
    scale_factor: f32,
    is_double: bool,
) -> bool {
    let leeway = 0.5;

    let manga_width = match is_double {
        true => DOUBLE_MANGA_WIDTH,
        false => MANGA_WIDTH,
    };

    let scaled_width = width * scale_factor;
    let scaled_height = height * scale_factor;

    (scaled_width - leeway) < manga_width && (scaled_height - leeway) < MANGA_HEIGHT
}
