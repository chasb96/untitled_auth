use std::{env, sync::OnceLock};
use log_unwrap::LogUnwrap;
use serde::Deserialize;

static CONFIGURATION: OnceLock<Configuration> = OnceLock::new();

#[derive(Deserialize)]
pub struct Configuration {
    pub database_url: String,
}

impl Configuration {
    pub fn configured() -> &'static Self {
        let config = CONFIGURATION
            .get_or_init(|| {
                let database_url = env::var("USERS_DATABASE_URL").log_unwrap();

                Configuration {
                    database_url,
                }
            });

        &config
    }
}