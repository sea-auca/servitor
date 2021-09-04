mod commands;

use commands::basic::*;
use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    //    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    // http::Http,
    model::{event::ResumedEvent, gateway::Ready, id::EmojiId, channel::{Reaction, ReactionType, Message}},
    prelude::*,
};
use std::env;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
    
    async fn message(&self, ctx: Context, msg: Message) {
        if &msg.content.chars().count() > &0 && &msg.content.chars().next().unwrap() == &'~'  {
            return
        }
        if msg.content.contains("Яман") || msg.content.contains("яман") {
            let reaction_type = ReactionType::Custom{animated: false, id: EmojiId(798454339134816256), name: Some(String::from(":shit_taster:"))};
            if let Err(why) = msg.react(&ctx, reaction_type).await {
                println!("Error reacting: {:?}", why);
            }
            if let Err(why) = msg.channel_id.say(&ctx.http, "Курлык-курлык!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        
    }
    
    
    
}

#[group]
#[commands(ping, echo, fortune, help, about)]
struct General;


#[tokio::main]
async fn main() {
    let token = env::var("BOT_TOKEN").expect("Bot token was not found");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(&token)
        .intents(GatewayIntents::all())
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        print!("Client error: {:?}\n", why);
    }
}
