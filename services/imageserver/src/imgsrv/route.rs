use crate::{
    errors::{map_io_error, map_lock_error, map_openslide_error, ImageServerError},
    imgsrv::{
        schemas::{
            EncodeType, EncodedImage, Roi, SlideInfo, ThumbnailRequestInput, TileRequestInput,
        },
        utils::{encore_buffer_rgba, get_slide_resolution},
    },
    settings::Settings,
};
use actix_web::{
    get, web,
    web::{Data, Json},
    Result,
};
use cached::proc_macro::cached;
use image::imageops::FilterType;
use openslide_rs::{errors::OpenSlideError, Address, DeepZoomGenerator, OpenSlide, Size};
use std::{
    cmp::min,
    fs,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, RwLock},
    time::Instant,
};

#[get("/slide/compatible_file_extensions")]
pub(crate) async fn compatible_file_extensions() -> Json<Vec<String>> {
    let extensions: Vec<String> = vec![".ndpi"].iter().map(|&s| s.into()).collect();
    Json(extensions)
}

#[get("/slide_size/{path}")]
pub(crate) async fn slide_size(path: String, state: Data<Settings>) -> Result<Json<f32>> {
    let path = state.imageserver.slide_dir.as_path().join(path);
    let size = fs::metadata(path).map_err(map_io_error)?.len();
    let size = size as f32 / 1024_u32.pow(3) as f32;
    Ok(Json(size))
}

#[get("/slide/{path}")]
pub(crate) async fn slide_info(
    state: Data<Settings>,
    path: web::Path<String>,
) -> Result<Json<SlideInfo>> {
    let path = path.into_inner();
    let path = state.imageserver.slide_dir.as_path().join(path);

    let cached_osr = get_osr(path).map_err(map_openslide_error)?;
    let osr = cached_osr.read().map_err(map_lock_error)?;
    // NOTE: here we set a dummy value for the tile size (254) because
    // the info (mpp_max, level_count, level_dimensions) do not depend
    // on this parameter
    let dz = DeepZoomGenerator::new(&osr, 254, 0, false).map_err(map_openslide_error)?;

    let level_dimensions = dz
        .level_dimensions()
        .iter()
        .map(|size| (size.w, size.h))
        .collect();

    let info = SlideInfo {
        mpp_max: get_slide_resolution(&osr),
        level_count: dz.level_count() as u16,
        level_dimensions,
    };

    Ok(Json(info))
}

#[get("/roi/{path}+{level}")]
pub(crate) async fn regions_of_interest(
    state: Data<Settings>,
    info: web::Path<(String, u32)>,
) -> Result<Json<Vec<Roi>>> {
    let (path, level) = info.into_inner();
    let path = state.imageserver.slide_dir.as_path().join(path);

    let cached_osr = get_osr(path).map_err(map_openslide_error)?;
    let osr = cached_osr.read().map_err(map_lock_error)?;
    // NOTE: here we set a dummy value for the tile size (254) because
    // the info (mpp_max, level_count, level_dimensions) do not depend
    // on this parameter
    let dz = DeepZoomGenerator::new(&osr, 254, 0, false).map_err(map_openslide_error)?;

    let level_dimensions = dz
        .level_dimensions()
        .get(level as usize)
        .ok_or(ImageServerError::InvalidSlideLevel)?;

    let roi = Roi {
        col: 0,
        row: 0,
        width: level_dimensions.w,
        height: level_dimensions.h,
    };

    Ok(Json(vec![roi]))
}

#[get("/tile/{path}+{col}+{row}+{tile_width}+{tile_height}+{level}+{format}+{quality}")]
pub(crate) async fn tile(
    state: Data<Settings>,
    info: web::Path<TileRequestInput>,
) -> Result<EncodedImage> {
    let input = info.into_inner();
    let path = state.imageserver.slide_dir.as_path().join(input.path);

    let tile_size = min(input.tile_width, input.tile_width);

    let start = Instant::now();
    let cached_osr = get_osr(path).map_err(map_openslide_error)?;
    let osr = cached_osr.read().map_err(map_lock_error)?;

    let dz = DeepZoomGenerator::new(&osr, tile_size, 0, false).map_err(map_openslide_error)?;
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);

    let offset = Address {
        x: input.col,
        y: input.row,
    };
    let start = Instant::now();
    let image = dz
        .get_tile(input.level, offset, FilterType::Lanczos3)
        .map_err(map_openslide_error)?;
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    let format = EncodeType::from_str(&input.format)?;

    let start = Instant::now();
    let encoded_image = encore_buffer_rgba(image, format, input.quality)?;
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    Ok(encoded_image)
}

#[get("/thumbnail/{path}+{width}+{height}+{level}+{format}+{quality}")]
pub(crate) async fn thumbnail(
    state: Data<Settings>,
    info: web::Path<ThumbnailRequestInput>,
) -> Result<EncodedImage> {
    let input = info.into_inner();
    let path = state.imageserver.slide_dir.as_path().join(input.path);

    let size = Size {
        w: input.width,
        h: input.height,
    };
    let cached_osr = get_osr(path).map_err(map_openslide_error)?;
    let osr = cached_osr.read().map_err(map_lock_error)?;

    let image = osr.thumbnail(&size).map_err(map_openslide_error)?;

    let format = EncodeType::from_str(&input.format)?;

    let encoded_image = encore_buffer_rgba(image, format, input.quality)?;
    Ok(encoded_image)
}

#[cached(size = 50, sync_writes = true)]
fn get_osr(path: PathBuf) -> Result<Arc<RwLock<OpenSlide>>, OpenSlideError> {
    let osr = OpenSlide::new(&path)?;
    Ok(Arc::new(RwLock::new(osr)))
}
