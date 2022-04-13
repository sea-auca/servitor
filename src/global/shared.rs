//! Stores object that are shared between modules and are intented to be singletons.
use crate::utilities::{db, logging};
use lazy_static::lazy_static;
use futures::lock;
lazy_static!  {
    pub static ref LOGGER: lock::Mutex<logging::Logger> = lock::Mutex::new(logging::Logger::new());
    pub static ref BOT_DATABASE: lock::Mutex<db::DatabaseClient> = lock::Mutex::new(db::DatabaseClient::new());
}
