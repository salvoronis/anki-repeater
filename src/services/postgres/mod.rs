use std::sync::{Arc, Mutex};
use shaku::{Interface, Component};
use crate::services::log::Logger;
use postgres::{Client, Error};

pub trait Postgres: Interface {
    fn init_tables(&self);
}

#[derive(Component)]
#[shaku(interface = Postgres)]
pub struct PostgresImpl {
    #[shaku(inject)]
    logger: Arc<dyn Logger>,
    client: Arc<Mutex<Client>>,
}

impl Postgres for PostgresImpl {
    fn init_tables(&self) {
        match self.create_pack() {
            Err(_) => self.logger.log("error when creating anki table"), // TODO add err
            Ok(_) => self.logger.log("created pack table"),
        }

        match self.create_anki() {
            Err(_) => self.logger.log("error when creating anki table"),
            Ok(_) => self.logger.log("created anki table"),
        }
    }
}

impl PostgresImpl {
    fn create_pack(&self) -> Result<u64, Error> {
        self.client.lock().unwrap()
            .execute(
                "CREATE TABLE IF NOT EXISTS pack (
                id              SERIAL PRIMARY KEY,
                name            VARCHAR NOT NULL
            )",
                &[],
            )
    }

    fn create_anki(&self) -> Result<u64, Error> {
        self.client.lock().unwrap()
            .execute(
                "CREATE TABLE IF NOT EXISTS anki  (
                id              SERIAL PRIMARY KEY,
                word            VARCHAR NOT NULL,
                reading         VARCHAR NOT NULL,
                meaning         VARCHAR NOT NULL,
                pack_id         INTEGER NOT NULL REFERENCES pack,
                repeat_count    INTEGER DEFAULT 0,
                first_time      DATE
            )",
                &[],
            )
    }
}