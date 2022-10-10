use crate::common;
use crate::imgsrv;
use crate::settings::Settings;
use anyhow::Error;
use rocket::{Build, Rocket};
use tracing::error;

pub async fn run(settings: &Settings) -> Result<(), Error> {
    if let Err(e) = build(settings)?.launch().await {
        error!("Whoops! Server didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    };
    Ok(())
}

fn build(settings: &Settings) -> Result<Rocket<Build>, Error> {
    let config = rocket::Config {
        address: settings.server.host.parse()?,
        port: settings.server.port,
        workers: settings.server.workers as usize,
        cli_colors: true,
        ..Default::default()
    };

    let rocket = rocket::custom(config)
        .mount("/", common::ROUTES.as_slice())
        .mount("/imgsrv/", imgsrv::ROUTES.as_slice())
        .manage(settings.clone());
    Ok(rocket)
}
