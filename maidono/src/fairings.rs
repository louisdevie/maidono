use crate::logger::Logger;
use crate::state::load_initial_actions;
use rocket::config::LogLevel;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{async_trait, Build, Rocket};

pub struct InitialActionsLoader();

#[async_trait]
impl Fairing for InitialActionsLoader {
    fn info(&self) -> Info {
        Info {
            kind: Kind::Ignite,
            name: "Initial actions loader",
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        let log_level = rocket
            .figment()
            .extract_inner("log_level")
            .unwrap_or(LogLevel::Critical);
        let logger = Logger::from(log_level);

        match load_initial_actions() {
            Ok(actions) => {
                logger.log("Successfully loaded actions");
                logger.debug(&actions);
                Ok(rocket.manage(actions))
            }
            Err(error) => {
                logger.error(error);
                Err(rocket)
            }
        }
    }
}
