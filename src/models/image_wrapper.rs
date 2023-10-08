use std::{fs::File, io::BufReader, path::Path};

use crate::models::{DPI, MANGA_HEIGHT};

use anyhow::{bail, Result};
use printpdf::{image_crate, Image};

const MM_PER_INCH: f32 = 25.4;
const MM_PER_DPI: f32 = MM_PER_INCH / DPI;

pub struct ImageWrapper {
    pub width_in_mm: f32,
    pub height_in_mm: f32,
    pub scale_factor_to_manga_heigth: f32,
    pub inner_image: Image,
}

impl ImageWrapper {
    pub fn new(image: &Path) -> Result<ImageWrapper> {
        let image = ImageWrapper::get_image(image)?;

        let height_in_mm = image.image.height.0 as f32 * MM_PER_DPI;
        Ok(ImageWrapper {
            width_in_mm: image.image.width.0 as f32 * MM_PER_DPI,
            height_in_mm: height_in_mm,
            scale_factor_to_manga_heigth: MANGA_HEIGHT / height_in_mm,
            inner_image: image,
        })
    }

    pub fn get_scaled_width_in_mm(&self) -> f32 {
        self.width_in_mm * self.scale_factor_to_manga_heigth
    }

    pub fn is_landscape(&self) -> bool {
        self.width_in_mm > self.height_in_mm
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
