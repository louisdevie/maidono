mod execution;
mod fairings;
mod handler;
mod hosts;
mod logger;
mod payload;
mod security;
mod state;
mod version;

use crate::fairings::InitialActionsLoader;
use crate::handler::WebhookHandler;
use maidono_core::problem;
use maidono_core::utils::path::{SERVER_CONFIG_FILE, WEB_APP_ASSETS, WEB_APP_INDEX};
use maidono_core::utils::Error;
use rocket::figment::providers::{Format, Serialized, Toml};
use rocket::figment::{Figment, Profile};
use rocket::fs::{FileServer, Options};

#[rocket::main]
async fn main() -> Result<(), Error> {
    println!(
        "===== maidono server version {}.{}.{} ====",
        version::MAJOR,
        version::MINOR,
        version::PATCH
    );
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::from(4471, Profile::Default).key("port"))
        .merge(Toml::file(SERVER_CONFIG_FILE).profile("default"))
        .merge((
            "ident",
            format!(
                "Maidono {}.{} webhook server",
                version::MAJOR,
                version::MINOR
            ),
        ));

    if let Err(error) = rocket::custom(figment)
        .mount(
            "/",
            FileServer::new(WEB_APP_INDEX, Options::IndexFile).rank(1),
        )
        .mount("/assets", FileServer::from(WEB_APP_ASSETS).rank(-1))
        .mount("/", WebhookHandler::routes())
        .attach(InitialActionsLoader())
        .launch()
        .await
    {
        Err(problem!("Failed to launch Rocket server").because(error.pretty_print()))
    } else {
        Ok(())
    }
}
