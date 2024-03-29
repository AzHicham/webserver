use crate::{common, errors::ImageServerError, imgsrv, settings::Settings};
use actix_web::{web, App, HttpServer};
use tracing::error;

pub async fn run(settings: &Settings) -> Result<(), ImageServerError> {
    if let Err(e) = build(settings).await {
        error!("Whoops! Server didn't launch!");
        drop(e);
    };
    Ok(())
}

async fn build(settings: &Settings) -> std::io::Result<()> {
    let settings = settings.clone();
    let host = settings.server.host.clone();
    let port = settings.server.port;
    let nb_workers = settings.server.workers;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(settings.clone()))
            .service(imgsrv::config())
            .service(common::config())
    })
    .bind((host, port))?
    .workers(nb_workers as usize)
    .run()
    .await
}
