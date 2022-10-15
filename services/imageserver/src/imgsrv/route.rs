use crate::common::schemas::ErrorMessage;
use crate::imgsrv::schemas::TileInfo;
use crate::{
    imgsrv::{errors::ImageServerError, schemas::Image},
    settings::Settings,
};
use actix_web::{
    get, web,
    web::{Data, Json},
};
use cached::proc_macro::cached;
use openslide_rs::{DeepZoomGenerator, Offset, OpenSlide};
use std::cmp::min;
use std::{
    fs,
    io::Cursor,
    path::PathBuf,
    sync::{Arc, RwLock},
};

#[get("/slide/compatible_file_extensions")]
pub(crate) async fn compatible_file_extensions() -> Json<Vec<String>> {
    let extensions: Vec<String> = vec![".ndpi"].iter().map(|&s| s.into()).collect();
    Json(extensions)
}

#[get("/slide_size/{path}")]
pub(crate) async fn slide_size(
    path: String,
    state: Data<Settings>,
) -> Result<Json<f32>, ImageServerError> {
    let path = state.imageserver.slide_dir.as_path().join(path);
    let size = fs::metadata(path)
        .map_err(|_| ImageServerError::IoError)?
        .len();
    let size = size as f32 / 1024_u32.pow(3) as f32;
    Ok(Json(size))
}
/*
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
}*/

#[get("/tile/{path}+{col}+{row}+{tile_width}+{tile_height}+{level}+{format}+{quality}")]
pub(crate) async fn tile(
    state: Data<Settings>,
    info: web::Path<TileInfo>,
) -> Result<Image, ImageServerError> {
    let input = info.into_inner();
    println!("{input:?}");
    let path = state.imageserver.slide_dir.as_path().join(input.path);

    let tile_size = min(input.tile_width, input.tile_width);

    let cached_osr = get_osr(path);
    let osr = cached_osr.read().unwrap();
    let deepzoom = DeepZoomGenerator::new(&*osr, tile_size, 0, false).unwrap();
    let image = deepzoom
        .get_tile(
            input.level,
            Offset {
                x: input.col,
                y: input.row,
            },
        )
        .expect("fsdfsdf");
    let mut default = vec![];
    use image::buffer::ConvertBuffer;
    let image: image::RgbImage = image.convert();

    use image::codecs::png::CompressionType;
    use image::codecs::png::FilterType;
    use image::codecs::png::PngEncoder;
    use image::ImageEncoder;

    let cursor = &mut Cursor::new(&mut default);
    let encoder = PngEncoder::new_with_quality(cursor, CompressionType::Fast, FilterType::NoFilter);
    encoder.write_image(
        image.as_raw().as_slice(),
        tile_size,
        tile_size,
        image::ColorType::Rgb8,
    );

    Ok(Image { data: default })
}

#[cached(sync_writes = true)]
fn get_osr(path: PathBuf) -> Arc<RwLock<OpenSlide>> {
    let osr = OpenSlide::new(&path).expect("dfsf");
    Arc::new(RwLock::new(osr))
}
