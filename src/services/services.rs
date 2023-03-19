use std::sync::{Arc, Mutex};
use postgres::Client;
use crate::services::log::{LoggerImpl};
use crate::services::postgres::{PostgresImpl, PostgresImplParameters};
use shaku::{module};

module! {
    pub CliAppModule {
        components = [LoggerImpl, PostgresImpl],
        providers = []
    }
}

pub fn cli_app_module(client: Arc<Mutex<Client>>) -> CliAppModule {
    CliAppModule::builder().with_component_parameters::<PostgresImpl>(PostgresImplParameters {
        client,
    }).build()
}