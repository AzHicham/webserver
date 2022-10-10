use anyhow::Error;
use imageserver::logger::init_logger;
use imageserver::server::run;
use imageserver::settings::Settings;
use tracing::debug;

#[rocket::main]
async fn main() -> Result<(), Error> {
    let settings = Settings::new()?;
    let _log_guard = init_logger(&settings);
    debug!("{:?}", settings);
    run(&settings).await
}
