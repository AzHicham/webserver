use anyhow::Error;
use tracing::debug;
use webserver::server::run;
use webserver::settings::Settings;

#[rocket::main]
async fn main() -> Result<(), Error> {
    let settings = Settings::new()?;
    debug!("{:?}", settings);
    run(&settings).await
}
