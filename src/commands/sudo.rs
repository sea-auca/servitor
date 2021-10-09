use serenity::framework::standard::{macros::command, macros::group, CommandResult, Args};
use serenity::model::{guild, id, prelude::*, user::*};
use serenity::prelude::*;
use crate::global::shared::{LOGGER, BOT_DATABASE};

#[group]
#[owners_only]
#[commands(sudo, retrieve_logs, add_reaction_role)]
struct Sudo;

#[command]
async fn sudo(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Looser!").await?;
    Ok(())
}

#[command]
#[num_args(1)]
async fn retrieve_logs(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let num_of_lines = args.single::<usize>()?;
    let dump = LOGGER.lock().unwrap().extract_entries(num_of_lines);
    msg.channel_id.say(&ctx.http, dump).await?;
    Ok(())
}


#[command]
#[num_args(3)]
#[usage("message_id emoji_id role_id")]
async fn add_reaction_role(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let msg_id = args.single::<String>()?;
    let emoji_id = args.single::<String>()?;
    let role_id = args.single::<String>()?;
    BOT_DATABASE.lock().unwrap().add_reaction_role(msg_id, emoji_id, role_id);
    msg.channel_id.say(&ctx.http, "Ok").await?;
    Ok(())
}