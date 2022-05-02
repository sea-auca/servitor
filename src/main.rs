//! # servitor
//!
//! `servitor` is a Discord bot helping to manage SEA Server.
mod config;
mod frameworks;
mod global;
mod handlers;
mod server;
mod utilities;


use config::setup;
use global::shared::LOGGER;
use utilities::logging;

#[tokio::main]
async fn main() {
    let settings =
        setup::Settings::create_settings()
            .await;
    let mut server = server::Server::create(settings)
        .await
        .expect("Error starting server");
    if let Err(why) = server.start().await {
        LOGGER
            .lock()
            .await
            .write_log(format!("Client error: {:?}\n", why), logging::Level::Error)
            .await;
    }
    
}
