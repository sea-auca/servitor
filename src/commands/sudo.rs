use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::{prelude::*, user::*, id, guild};
use serenity::prelude::*;

#[command]
async fn sudo(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Looser!")
        .await?;
    Ok(())
}