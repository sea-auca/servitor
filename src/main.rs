mod commands;
mod config;
mod handlers;

use commands::basic::*;
use config::setup;
use handlers::handler;
use serenity::{
    client::bridge::gateway::GatewayIntents,
    //    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    // http::Http,
    prelude::*,
};
use std::env;

#[group]
#[commands(ping, echo, fortune, help, about)]
struct General;

#[tokio::main]
async fn main() {
    let settings =
        setup::Settings::create_settings(String::from("data/Config.toml"), &GENERAL_GROUP);
    let mut client = Client::builder(settings.config.get_token())
        .intents(GatewayIntents::all())
        .framework(settings.framework)
        .event_handler(handler::Handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        print!("Client error: {:?}\n", why);
    }
}
