use crate::settings::Settings;
use crate::{authentication, common};
use anyhow::Error;
use celery::broker::RedisBroker;
use celery::Celery;
use rocket::{Build, Rocket};
use std::sync::Arc;
use tracing::error;

pub async fn run(settings: &Settings) -> Result<(), Error> {
    let celery = init_celery(settings).await?;

    if let Err(e) = build(settings)?.manage(celery).launch().await {
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
        .mount("/oauth2", authentication::ROUTES.as_slice());
    // .register("/", catchers![general_not_found, default_catcher])
    Ok(rocket)
}

async fn init_celery(settings: &Settings) -> celery::export::Result<Arc<Celery<RedisBroker>>> {
    celery::app!(
        broker = RedisBroker { &settings.broker.address },
        tasks = [
        ],
        task_routes = [
            "buggy_task" => "buggy-queue",
            "*" => "celery",
        ],
        prefetch_count = 2,
        heartbeat = Some(10),
    )
    .await
}
