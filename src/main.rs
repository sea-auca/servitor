mod commands;
mod config;
mod frameworks;
mod handlers;

use commands::HELP;
use commands::basic::GENERAL_GROUP;
use commands::basic_member::BASICMEMBER_GROUP;
use commands::sudo::SUDO_GROUP;
use config::setup;
use serenity::prelude::*;
#[tokio::main]
async fn main() {
    let settings = setup::Settings::create_settings(
        String::from("data/Config.toml"),
        &vec![&GENERAL_GROUP, &BASICMEMBER_GROUP, &SUDO_GROUP],
        &HELP
    );
    let mut client = Client::builder(settings.config.get_token())
        .intents(settings.intents)
        .framework(settings.framework)
        .event_handler(settings.handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        print!("Client error: {:?}\n", why);
    }
}
