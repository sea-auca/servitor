
use crate::utilities::{logging, db};
use std::sync::{Mutex};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref LOGGER: Mutex<logging::Logger> = Mutex::new(logging::Logger::new());
    pub static ref BOT_DATABASE: Mutex<db::Client> = Mutex::new(db::Client::new());
}

