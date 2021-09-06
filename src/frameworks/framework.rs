use serenity::{
    framework::standard::{macros::hook, CommandResult, StandardFramework},
    model::{
        channel::{Message, ReactionType},
        id::{EmojiId, UserId},
    },
    client::Context
};
use std::{collections::HashSet};
use crate::global::shared::LOGGER;
use crate::utilities::logging::Level;


pub fn create_framework(prefix: &str) -> StandardFramework {
    let mut owners: HashSet<UserId> = HashSet::new();
    owners.insert(UserId(694042020100046990));
    owners.insert(UserId(509346504423637006));
    StandardFramework::new()
        .configure(|c| c.prefix(prefix).owners(owners))
        .before(before)
        .after(after)
        .normal_message(normal_message)
}

#[hook]
async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
   // if msg.author.bot {
   //     return false;
   // }
    
    LOGGER.lock().unwrap().write_log(format!("Got command '{}' by user '{}'",command_name, msg.author.name), Level::Trace);
    true
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => {
            LOGGER.lock().unwrap().write_log(format!("Processed command '{}'", command_name), Level::Trace);
        }
        Err(why) => {
            LOGGER.lock().unwrap().write_log(format!("Command '{}' returned error {:?}.", command_name, why), Level::Warning);
        }
    }
}

#[hook]
async fn normal_message(ctx: &Context, msg: &Message) {
    LOGGER.lock().unwrap().write_log(format!("Received message from user {}", msg.author.name), Level::Trace);
    if msg.content.contains("Яман") || msg.content.contains("яман") {
        let reaction_type = ReactionType::Custom {
            animated: false,
            id: EmojiId(798454339134816256),
            name: Some(String::from(":shit_taster:")),
        };
        if let Err(why) = msg.react(&ctx, reaction_type).await {
            LOGGER.lock().unwrap().write_log(format!("Error adding reaction {:?}", why), Level::Warning);
        }
        if let Err(why) = msg.channel_id.say(&ctx.http, "Курлык-курлык!").await {            
            LOGGER.lock().unwrap().write_log(format!("Error sending message {:?}", why), Level::Warning);
        }
    }
}


