use crate::utilities::{db, logging};
use lazy_static::lazy_static;
use std::sync::Mutex;
lazy_static! {
    pub static ref LOGGER: Mutex<logging::Logger> = Mutex::new(logging::Logger::new());
    pub static ref BOT_DATABASE: Mutex<db::DatabaseClient> = Mutex::new(db::DatabaseClient::new());
}
