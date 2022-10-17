use imageserver::{errors::ImageServerError, logger::init_logger, server::run, settings::Settings};

#[actix_web::main]
async fn main() -> Result<(), ImageServerError> {
    let settings = Settings::new().map_err(|_| ImageServerError::ConfigError)?;
    init_logger(&settings);
    run(&settings).await
}
