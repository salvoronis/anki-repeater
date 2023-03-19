use std::sync::Arc;
use shaku::{Interface, Component};
use crate::services::log::Logger;
use redis::{Client, RedisResult};

pub trait Redis: Interface {
    fn redis_test(&self) -> RedisResult<()>;
}

#[derive(Component)]
#[shaku(interface = Redis)]
pub struct RedisImpl {
    #[shaku(inject)]
    logger: Arc<dyn Logger>,
    client: Client,
}

impl Redis for RedisImpl {
    fn redis_test(&self) -> RedisResult<()> {
        let mut conn = self.client.get_connection()
            .expect("cannot get connection to redis");

        redis::cmd("SET")
            .arg("foo")
            .arg("bar")
            .query(&mut conn)
    }
}