use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::process::Command;

const ABOUT_MSG: &str = "Hello! This is simple utility bot developed by our community. 
    We are still in process of development and new features will be added later";

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn echo(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, &msg.content).await?;
    Ok(())
}

#[command]
async fn fortune(ctx: &Context, msg: &Message) -> CommandResult {
    let output = Command::new("/usr/games/fortune")
        .output()
        .expect("Failed to execute fortune");
    msg.channel_id
        .say(&ctx.http, String::from_utf8_lossy(&output.stdout))
        .await?;
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "No help for you!").await?;
    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, ABOUT_MSG).await?;
    Ok(())
}
