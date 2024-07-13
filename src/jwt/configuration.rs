use std::{env, sync::OnceLock};
use log_unwrap::LogUnwrap;
use serde::Deserialize;

static CONFIGURATION: OnceLock<Configuration> = OnceLock::new();

#[derive(Deserialize)]
pub struct Configuration {
    pub hmac_key: String,
}

impl Configuration {
    pub fn configured() -> &'static Self {
        let config = CONFIGURATION
            .get_or_init(|| {
                let hmac_key = env::var("JWT_HMAC_KEY").log_unwrap();

                Configuration {
                    hmac_key,
                }
            });

        &config
    }
}