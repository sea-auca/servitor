
use crate::utilities::logging;
use std::sync::{Mutex};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref LOGGER: Mutex<logging::Logger> = Mutex::new(logging::Logger::new());    
}

