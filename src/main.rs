mod commands;
mod config;
mod handlers;

use commands::basic::*;
use config::setup;
use serenity::{
    //    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group},
    // http::Http,
    prelude::*,
};

#[group]
#[commands(ping, echo, fortune, help, about)]
struct General;

#[tokio::main]
async fn main() {
    let settings =
        setup::Settings::create_settings(String::from("data/Config.toml"), &GENERAL_GROUP);
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
