use anyhow::Error;
use imageserver::{logger::init_logger, server::run, settings::Settings};
use tracing::debug;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let settings = Settings::new()?;
    let _ = init_logger(&settings);
    debug!("{:?}", settings);
    run(&settings).await
}
