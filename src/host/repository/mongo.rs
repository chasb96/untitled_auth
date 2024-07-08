use std::{error::Error, fmt::Display, sync::OnceLock};

use deadpool::managed::{BuildError, Pool};
use futures::executor::block_on;
use log_unwrap::LogUnwrap;

use crate::host::configuration::Configuration;

use super::deadpool::MongoConnectionManager;

static CONNECTION_POOL: OnceLock<Pool<MongoConnectionManager>> = OnceLock::new();

#[derive(Debug)]
pub enum InitializeConnectionPoolError {
    Mongo(mongodb::error::Error),
    Deadpool(BuildError),
}

impl Error for InitializeConnectionPoolError { }

impl Display for InitializeConnectionPoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitializeConnectionPoolError::Mongo(e) => write!(f, "InitializeConnectionPoolError::Mongo({})", e),
            InitializeConnectionPoolError::Deadpool(e) => write!(f, "InitializeConnectionPoolError::Deadpool({})", e),
        }
    }
}

impl From<mongodb::error::Error> for InitializeConnectionPoolError {
    fn from(value: mongodb::error::Error) -> Self {
        Self::Mongo(value)
    }
}

impl From<BuildError> for InitializeConnectionPoolError {
    fn from(value: BuildError) -> Self {
        Self::Deadpool(value)
    }
}

pub struct MongoDatabase {
    pub connection_pool: &'static Pool<MongoConnectionManager>,
}

impl Default for MongoDatabase {
    fn default() -> Self {
        Self {
            connection_pool: CONNECTION_POOL
                .get_or_init(|| {
                    block_on(async {
                        let configuration = Configuration::configured();

                        let manager = MongoConnectionManager {
                            client: mongodb::Client::with_uri_str(&configuration.database_url)
                                .await
                                .log_unwrap(),
                            database: configuration.database_name
                                .clone()
                                .unwrap()
                                .to_string(),
                        };
    
                        Pool::builder(manager)
                            .build()
                            .log_unwrap()
                    })
                })
        }
    }
}