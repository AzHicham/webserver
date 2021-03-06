use anyhow::Error;
use tracing::debug;
use webserver::logger::init_logger;
use webserver::server::run;
use webserver::settings::Settings;

#[rocket::main]
async fn main() -> Result<(), Error> {
    let settings = Settings::new()?;
    let _log_guard = init_logger(&settings);
    debug!("{:?}", settings);
    run(&settings).await
}
