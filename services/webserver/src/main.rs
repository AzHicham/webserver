use anyhow::Error;
use tracing::debug;
use webserver::{logger::init_logger, server::run, settings::Settings};

#[rocket::main]
async fn main() -> Result<(), Error> {
    let settings = Settings::new()?;
    init_logger(&settings);
    debug!("{:?}", settings);
    run(&settings).await
}
