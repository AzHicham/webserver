use crate::imgsrv::schemas::{
    EncodeType, EncodedImage, Roi, SlideInfo, ThumbnailRequestInput, TileRequestInput,
};
use crate::imgsrv::utils::{encore_buffer_rgba, get_slide_resolution, get_thumbnail_helper};
use crate::{imgsrv, imgsrv::errors::ImageServerError, settings::Settings};
use actix_web::{
    get, web,
    web::{Data, Json},
};
use cached::proc_macro::cached;
use image::imageops::FilterType;
use openslide_rs::{DeepZoomGenerator, Offset, OpenSlide, Size};
use std::cmp::min;
use std::str::FromStr;
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

#[get("/slide/{path}")]
pub(crate) async fn slide_info(
    state: Data<Settings>,
    path: web::Path<String>,
) -> Result<Json<SlideInfo>, ImageServerError> {
    let path = path.into_inner();
    let path = state.imageserver.slide_dir.as_path().join(path);

    let cached_osr = get_osr(path)?;
    let osr = cached_osr.read().map_err(|_| ImageServerError::IoError)?;
    // NOTE: here we set a dummy value for the tile size (254) because
    // the info (mpp_max, level_count, level_dimensions) do not depend
    // on this parameter
    let dz = DeepZoomGenerator::new(&osr, 254, 0, false).map_err(|_| ImageServerError::IoError)?;

    let level_dimensions = dz
        .level_dimensions()
        .iter()
        .map(|size| (size.width, size.height))
        .collect();

    let info = SlideInfo {
        mpp_max: get_slide_resolution(&osr),
        level_count: dz
            .level_count()
            .try_into()
            .map_err(|_| ImageServerError::IoError)?,
        level_dimensions,
    };

    Ok(Json(info))
}

#[get("/roi/{path}+{level}")]
pub(crate) async fn regions_of_interest(
    state: Data<Settings>,
    info: web::Path<(String, u32)>,
) -> Result<Json<Vec<Roi>>, ImageServerError> {
    let (path, level) = info.into_inner();
    let path = state.imageserver.slide_dir.as_path().join(path);

    let cached_osr = get_osr(path)?;
    let osr = cached_osr.read().map_err(|_| ImageServerError::IoError)?;
    // NOTE: here we set a dummy value for the tile size (254) because
    // the info (mpp_max, level_count, level_dimensions) do not depend
    // on this parameter
    let dz = DeepZoomGenerator::new(&osr, 254, 0, false).map_err(|_| ImageServerError::IoError)?;

    let level_dimensions = dz
        .level_dimensions()
        .get(level as usize)
        .ok_or(ImageServerError::IoError)?;

    let roi = Roi {
        col: 0,
        row: 0,
        width: level_dimensions.width,
        height: level_dimensions.height,
    };

    Ok(Json(vec![roi]))
}

#[get("/tile/{path}+{col}+{row}+{tile_width}+{tile_height}+{level}+{format}+{quality}")]
pub(crate) async fn tile(
    state: Data<Settings>,
    info: web::Path<TileRequestInput>,
) -> Result<EncodedImage, ImageServerError> {
    let input = info.into_inner();
    let path = state.imageserver.slide_dir.as_path().join(input.path);

    let tile_size = min(input.tile_width, input.tile_width);

    let cached_osr = get_osr(path)?;
    let osr = cached_osr.read().map_err(|_| ImageServerError::IoError)?;

    let dz =
        DeepZoomGenerator::new(&osr, tile_size, 0, false).map_err(|_| ImageServerError::IoError)?;

    let offset = Offset {
        x: input.col,
        y: input.row,
    };

    let image = dz
        .get_tile(input.level, offset, FilterType::Lanczos3)
        .map_err(|_| ImageServerError::IoError)?;

    let format = EncodeType::from_str(&input.format).map_err(|_| ImageServerError::IoError)?;

    encore_buffer_rgba(image, format, input.quality)
}

#[get("/thumbnail/{path}+{width}+{height}+{level}+{format}+{quality}")]
pub(crate) async fn thumbnail(
    state: Data<Settings>,
    info: web::Path<ThumbnailRequestInput>,
) -> Result<EncodedImage, ImageServerError> {
    let input = info.into_inner();
    let path = state.imageserver.slide_dir.as_path().join(input.path);

    let size = Size {
        width: input.width,
        height: input.height,
    };

    let cached_osr = get_osr(path)?;
    let osr = cached_osr.read().map_err(|_| ImageServerError::IoError)?;

    let (offset, level, level_size) = get_thumbnail_helper(&osr, &size)?;

    let image = osr
        .read_image(&offset, level, &level_size)
        .map_err(|_| ImageServerError::IoError)?;

    let image = image::imageops::thumbnail(&image, size.width, size.height);

    let format = EncodeType::from_str(&input.format).map_err(|_| ImageServerError::IoError)?;

    encore_buffer_rgba(image, format, input.quality)
}

#[cached(sync_writes = true)]
fn get_osr(path: PathBuf) -> Result<Arc<RwLock<OpenSlide>>, ImageServerError> {
    let osr = OpenSlide::new(&path).map_err(|_| ImageServerError::IoError)?;
    Ok(Arc::new(RwLock::new(osr)))
}
