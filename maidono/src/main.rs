use maidono_core::utils::path::SERVER_CONFIG_FILE;
use rocket::figment::providers::{Format, Serialized, Toml};
use rocket::figment::{Figment, Profile};
use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::from(4471, Profile::Default).key("port"))
        .merge(Toml::file(SERVER_CONFIG_FILE).nested())
        .merge(("ident", "Maidono 0.1 webhook server"))
        .select("webhooks");

    rocket::custom(figment).mount("/", routes![index])
}
