use std::sync::{Arc, Mutex};
use postgres::Client;
use crate::services::log::{LoggerImpl};
use crate::services::postgres::{PostgresImpl, PostgresImplParameters};
use shaku::{module};
use redis::{Client as RedisClient};
use crate::services::redis::{RedisImpl, RedisImplParameters};

module! {
    pub CliAppModule {
        components = [LoggerImpl, PostgresImpl, RedisImpl],
        providers = []
    }
}

pub fn cli_app_module(client: Arc<Mutex<Client>>, redis_client: RedisClient) -> CliAppModule {
    CliAppModule::builder()
        .with_component_parameters::<PostgresImpl>(PostgresImplParameters {
            client,
        })
        .with_component_parameters::<RedisImpl>(RedisImplParameters {
            client: redis_client,
        })
        .build()
}