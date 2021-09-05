
use crate::utilities::logging;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref LOGGER: Arc<Mutex<logging::Logger>> = Arc::new(Mutex::new(logging::Logger::new()));    
}

