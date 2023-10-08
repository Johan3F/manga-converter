use std::{fs::File, io::BufReader, path::Path};

use crate::models::{DOUBLE_MANGA_WIDTH, DPI, MANGA_HEIGHT, MANGA_WIDTH};

use anyhow::{bail, Result};
use printpdf::{image_crate, Image};

const MM_PER_INCH: f32 = 25.4;
const MM_PER_DPI: f32 = MM_PER_INCH / DPI;

const COMPARISSON_LEEWAY: f32 = 0.5;

pub struct ImageWrapper {
    pub width_in_mm: f32,
    pub height_in_mm: f32,
    pub inner_image: Image,
}

impl ImageWrapper {
    pub fn new(image: &Path) -> Result<ImageWrapper> {
        let image = ImageWrapper::get_image(image)?;
        Ok(ImageWrapper {
            width_in_mm: image.image.width.0 as f32 * MM_PER_DPI,
            height_in_mm: image.image.height.0 as f32 * MM_PER_DPI,
            inner_image: image,
        })
    }

    pub fn get_scale_factor(&self) -> Result<f32> {
        let width_scale_factor = match self.height_in_mm > self.width_in_mm {
            true => MANGA_WIDTH / self.width_in_mm,
            false => DOUBLE_MANGA_WIDTH / self.width_in_mm,
        };

        let mut scale_factor = width_scale_factor;
        if !self.is_in_manga_bounds_after_scale(width_scale_factor) {
            scale_factor = MANGA_HEIGHT / self.height_in_mm;
        }

        return Ok(scale_factor);
    }

    pub fn is_landscape(&self) -> bool {
        self.width_in_mm > self.height_in_mm
    }

    fn is_in_manga_bounds_after_scale(&self, scale_factor: f32) -> bool {
        let manga_width = match self.is_landscape() {
            true => DOUBLE_MANGA_WIDTH,
            false => MANGA_WIDTH,
        };

        let corrected_width = (self.width_in_mm * scale_factor) - COMPARISSON_LEEWAY;
        let corrected_heigth = (self.height_in_mm * scale_factor) - COMPARISSON_LEEWAY;

        corrected_width < manga_width && corrected_heigth < MANGA_HEIGHT
    }

    fn get_image(image_path: &Path) -> Result<Image> {
        if image_path.extension().is_none() || image_path.extension().unwrap().to_str().is_none() {
            bail!("file image extension is not found or recognized");
        }

        let image_extension = image_path.extension().unwrap().to_str().unwrap();

        let image_file = File::open(image_path).unwrap();
        let image_buffer = BufReader::new(image_file);

        let image = match image_extension {
            "png" => Image::try_from(image_crate::codecs::png::PngDecoder::new(image_buffer)?)?,
            "jpg" => Image::try_from(image_crate::codecs::jpeg::JpegDecoder::new(image_buffer)?)?,
            _ => bail!("image type not supported"),
        };

        Ok(image)
    }
}
