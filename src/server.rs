use crate::api::{hello, world};
use crate::settings::Settings;
use anyhow::Error;
use rocket::{routes, Build, Rocket};
use tracing::error;

pub async fn run(settings: &Settings) -> Result<(), Error> {
    //let _log_guard = logger_init().map_err(|err| Error::InitLog { source: err })?;

    if let Err(e) = build(settings)?.launch().await {
        error!("Whoops! Server didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    };
    Ok(())
}

pub fn build(settings: &Settings) -> Result<Rocket<Build>, Error> {
    let config = rocket::Config {
        address: settings.server.host.parse()?,
        port: settings.server.port,
        workers: settings.server.workers as usize,
        cli_colors: true,
        ..Default::default()
    };

    let rocket = rocket::custom(config).mount("/", routes![world, hello]);
    // .register("/", catchers![general_not_found, default_catcher])
    Ok(rocket)
}
