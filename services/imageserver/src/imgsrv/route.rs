use crate::imgsrv::errors::{from_io_error, ImageServerError};
use crate::imgsrv::schemas::{Image, SlideInfo};
use crate::settings::Settings;
use anyhow::{bail, Error};
use image::buffer::ConvertBuffer;
use image::codecs::png::CompressionType;
use image::codecs::png::FilterType;
use image::codecs::png::PngEncoder;
use image::GrayImage;
use image::ImageEncoder;
use image::RgbaImage;
use openslide::bindings::read_region;
use openslide::{bindings, OpenSlide};
use rocket::serde::json::Json;
use rocket::State;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

#[get("/slide/compatible_file_extensions")]
pub(crate) async fn compatible_file_extensions() -> Json<Vec<String>> {
    let extensions: Vec<String> = vec![".ndpi"].iter().map(|&s| s.into()).collect();
    Json(extensions)
}

#[get("/slide_size/<path>")]
pub(crate) async fn slide_size(
    state: &State<Settings>,
    path: String,
) -> Result<Json<f32>, ImageServerError> {
    let path = state.imageserver.slide_dir.as_path().join(path);
    let size = fs::metadata(path).map_err(from_io_error)?.len();
    let size = size as f32 / 1024_u32.pow(3) as f32;
    Ok(Json(size))
}

#[get("/slide/<path>")]
pub(crate) async fn slide(
    state: &State<Settings>,
    path: String,
) -> Result<Json<SlideInfo>, ImageServerError> {
    let path = state.imageserver.slide_dir.as_path().join(path);
    let osr = get_osr(path).unwrap();

    let mut level_dimensions = vec![];

    for i in 0..osr.get_level_count().unwrap() {
        let dims = osr.get_level_dimensions(i).unwrap();
        level_dimensions.push((dims.0 as u32, dims.1 as u32));
    }

    let info = SlideInfo {
        mpp_max: Some(osr.get_level_count().unwrap() as f32),
        level_count: osr.get_level_count().unwrap() as u16,
        level_dimensions,
    };

    Ok(Json(info))
}

#[get("/tile/<path>")]
pub(crate) async fn thumbnail(
    state: &State<Settings>,
    path: String,
) -> Result<Image, ImageServerError> {
    let path = state.imageserver.slide_dir.as_path().join(path);
    let osr = bindings::open(path.to_str().unwrap()).unwrap();
    let image = bindings::read_region(osr, 0, 0, 0, 512, 512).unwrap();
    let image = vec_u32_to_u8(&image);
    let mut default = vec![];
    // img.write_to(&mut Cursor::new(&mut default), image::ImageFormat::Png);
    let cursor = &mut Cursor::new(&mut default);
    let encoder = PngEncoder::new_with_quality(cursor, CompressionType::Fast, FilterType::NoFilter);
    encoder.write_image(image.as_slice(), 512, 512, image::ColorType::Rgba8);
    Ok(Image { data: default })
}

pub fn vec_u32_to_u8(data: &Vec<u32>) -> Vec<u8> {
    // TODO: https://stackoverflow.com/questions/72631065/how-to-convert-a-u32-array-to-a-u8-array-in-place
    // TODO: https://stackoverflow.com/questions/29037033/how-to-slice-a-large-veci32-as-u8
    let capacity = 32 / 8 * data.len() as usize; // 32/8 == 4
    let mut output = Vec::<u8>::with_capacity(capacity);
    for &value in data {
        output.push((value >> 16) as u8); // r
        output.push((value >> 8) as u8); // g
        output.push((value >> 0) as u8); // b
        output.push((value >> 24) as u8); // a
    }
    output
}

fn get_osr(filename: PathBuf) -> Result<OpenSlide, Error> {
    let os = OpenSlide::new(&filename).ok();
    if let Some(os) = os {
        Ok(os)
    } else {
        bail!("Error")
    }
}
