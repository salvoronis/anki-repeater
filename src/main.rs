mod services;

use postgres::{Client, NoTls};

use shaku::{HasComponent};
use std::sync::{Arc, Mutex};

fn main() {
    let client = Client::connect(
        "postgresql://salvoroni:@localhost/anki",
        NoTls).expect("cannot connect to postgres");

    let module = services::services::cli_app_module(Arc::new(Mutex::new(client)));

    let postgres: &dyn services::postgres::Postgres = module.resolve_ref();

    postgres.init_tables();
}