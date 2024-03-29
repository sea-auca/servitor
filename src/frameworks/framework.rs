//! Implementation of standard framework for the bot.
use crate::global::shared::LOGGER;
use crate::utilities::logging::Level;
use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandResult, StandardFramework},
    model::{
        channel::{Message, ReactionType},
        id::{EmojiId, UserId},
    },
};
use std::collections::HashSet;

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

    LOGGER
        .lock()
        .await.
        write_log(
            format!(
                "Got command '{}' by user '{}'",
                command_name, msg.author.name
            ),
            Level::Trace,
        )
        .await;
    true
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => {
            LOGGER
                .lock()
                .await
                .write_log(
                    format!("Processed command '{}'", command_name),
                    Level::Trace,
                )
                .await;
        }
        Err(why) => {
            LOGGER.
                lock().
                await.
                write_log(
                    format!("Command '{}' returned error {:?}.", command_name, why),
                    Level::Warning,
                )
                .await;
        }
    }
}

#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    LOGGER
        .lock()
        .await
        .write_log(
            format!("Received message from user {}", msg.author.name),
            Level::Trace,
        )
        .await;
}
