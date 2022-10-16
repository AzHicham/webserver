use crate::imgsrv::errors::ImageServerError;
use crate::imgsrv::schemas::{EncodeType, EncodedImage};
use image::codecs::jpeg;
use image::codecs::png;
use image::RgbaImage;
use image::{ImageEncoder, ImageResult};
use openslide_rs::{DeepZoomGenerator, Offset, OpenSlide, Size};
use std::cmp::max;
use std::io::Cursor;

pub fn encore_buffer_rgba(
    image: RgbaImage,
    format: EncodeType,
    quality: u8,
) -> Result<EncodedImage, ImageServerError> {
    let mut data = vec![];

    let cursor = &mut Cursor::new(&mut data);

    match format {
        EncodeType::Png => {
            let encoder = png::PngEncoder::new_with_quality(
                cursor,
                png::CompressionType::Fast,
                png::FilterType::NoFilter,
            );
            encoder
                .write_image(
                    image.as_raw().as_slice(),
                    image.width(),
                    image.height(),
                    image::ColorType::Rgba8,
                )
                .map_err(|_| ImageServerError::IoError)?;
        }
        EncodeType::Jpeg => {
            let encoder = jpeg::JpegEncoder::new_with_quality(cursor, quality);
            encoder
                .write_image(
                    image.as_raw().as_slice(),
                    image.width(),
                    image.height(),
                    image::ColorType::Rgba8,
                )
                .map_err(|_| ImageServerError::IoError)?;
        }
    };

    Ok(EncodedImage { data, format })
}

pub fn get_slide_resolution(osr: &OpenSlide) -> Option<f32> {
    let mpp_x = osr.properties.openslide_properties.mpp_x;
    let mpp_y = osr.properties.openslide_properties.mpp_x;

    let x_res = osr.properties.tiff_properties.x_resolution;
    let res_unit = &osr.properties.tiff_properties.resolution_unit;

    match (mpp_x, mpp_y, x_res, res_unit) {
        (Some(mpp_x), Some(mpp_y), _, _) => Some((mpp_x + mpp_y) / 2.0),
        (_, _, Some(x_res), _) if x_res < 1_f32 => Some(x_res),
        (_, _, Some(x_res), Some(res_unit)) => {
            let factor = match res_unit.as_str() {
                "cm" | "centimeter" => 10000_f32,
                "mm" | "millimeter" => 1000_f32,
                "inch" => 25400_f32,
                _ => return None,
            };
            Some(factor / x_res)
        }
        _ => None,
    }
}

pub fn get_thumbnail_helper(
    osr: &OpenSlide,
    size: &Size,
) -> Result<(Offset, u32, Size), ImageServerError> {
    let dimension_level0 = osr
        .get_level0_dimensions()
        .map_err(|_| ImageServerError::IoError)?;

    let downsample = (
        dimension_level0.width as f64 / size.width as f64,
        dimension_level0.height as f64 / size.height as f64,
    );
    let downsample = f64::max(downsample.0, downsample.1);

    let level = osr
        .get_best_level_for_downsample(downsample)
        .map_err(|_| ImageServerError::IoError)?;

    let size = osr
        .get_level_dimensions(level)
        .map_err(|_| ImageServerError::IoError)?;

    let offset = Offset { x: 0, y: 0 };
    Ok((offset, level, size))
}
