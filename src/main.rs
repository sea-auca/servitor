mod commands;

use std::{env};
use serenity::{
    async_trait,
//    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    client::bridge::gateway::GatewayIntents,
    http::Http,
    model::{event::ResumedEvent, gateway::Ready,},
    prelude::*,
};

#[tokio::main]
async fn main() {
    let token = env::var("BOT_TOKEN").expect("Bot token was not found");
    let mut client = Client::builder(&token)
        .intents(GatewayIntents::all());
}
