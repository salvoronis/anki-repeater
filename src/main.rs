mod services;

use postgres::{Client, NoTls};

use shaku::{HasComponent};
use std::sync::{Arc, Mutex};
use redis::{Client as RedisClient};

fn main() {
    let client = Client::connect(
        "postgresql://salvoroni:@localhost/anki",
        NoTls).expect("cannot connect to postgres");

    let redis_client = RedisClient::open("redis://127.0.0.1/")
        .expect("cannot connect to redis");
        // .get_connection()
        // .expect("cannot get connection to redis");

    let module = services::services::cli_app_module(
        Arc::new(Mutex::new(client)),
        redis_client,
    );

    let postgres: &dyn services::postgres::Postgres = module.resolve_ref();
    let redis: &dyn services::redis::Redis = module.resolve_ref();

    postgres.init_tables();
    redis.redis_test().expect("TODO: panic message");
}