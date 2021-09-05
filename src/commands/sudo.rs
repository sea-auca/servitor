use serenity::framework::standard::{macros::command, macros::group, CommandResult};
use serenity::model::{guild, id, prelude::*, user::*};
use serenity::prelude::*;

#[group]
#[owners_only]
#[commands(sudo)]
struct Sudo;

#[command]
async fn sudo(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Looser!").await?;
    Ok(())
}
