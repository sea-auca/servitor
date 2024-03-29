use crate::global::shared::{BOT_DATABASE, LOGGER};
use serenity::framework::standard::{macros::command, macros::group, Args, CommandResult};
use serenity::model::{guild, id, prelude::*, user::*, channel::GuildChannel};
use serenity::prelude::*;
use serenity::builder;
use crate::utilities::logging::Level;

#[group]
#[owners_only]
#[commands(sudo, retrieve_logs, add_reaction_role, purge_channel)]
struct Sudo;

#[command]
async fn sudo(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Looser!").await?;
    Ok(())
}

#[command]
#[max_args(1)]
async fn retrieve_logs(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() == 1 {
        let num_of_lines = args.single::<usize>()?;
        let dump = LOGGER.lock().await.extract_entries(num_of_lines).await;
        msg.channel_id.say(&ctx.http, dump).await?;
        return Ok(())
    }
    let filepath = LOGGER.lock().await.get_logfile_path();
    match filepath
    {
        None => {
            msg.channel_id.say(&ctx.http, "No logfile exist").await?;
            Ok(())
        }
        Some(filepath) => {
            let body = String::from("The logfile is attached!");
            msg.channel_id.send_message(&ctx.http, |m| {
                m.content(body);
                m.add_file(filepath.as_str());
                m
            }).await?;
            Ok(())
        }
        
    }
}

#[command]
#[num_args(3)]
#[usage("message_id emoji_id role_id")]
async fn add_reaction_role(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let msg_id = args.single::<String>()?;
    let emoji_id = args.single::<String>()?;
    let role_id = args.single::<String>()?;
    BOT_DATABASE
        .lock()
        .await
        .add_reaction_role(msg_id, emoji_id, role_id)
        .await;
    msg.channel_id.say(&ctx.http, "Ok").await?;
    Ok(())
}

#[command]
#[num_args(0)]
#[only_in(guilds)]
#[description("ATTENTION UNSAFE FUNCTION. Deletes every single message found in the channel where message was sent.")]
async fn purge_channel(ctx: &Context, msg: &Message) -> CommandResult {
    LOGGER.lock().await.write_log(format!("Starting purging channel {}", msg.channel_id.0), Level::Debug).await;
    let channel = msg.channel(&ctx.cache).await.unwrap();
    if let Channel::Guild(channel) = channel {
        let messages = channel.messages(&ctx.http, |retriever| {
            retriever.before(msg.id)}).await;
        if let Ok(mut messages) = messages {
            for message in messages.iter_mut() {
                let result = message.delete(&ctx.http).await;
                if let Ok(_) = result {
                    LOGGER.lock().await.write_log(format!("Deleted message {}", message.id.0), Level::Debug).await;
                }
                else {
                    LOGGER.lock().await.write_log(format!("Failed to delete message {}", message.id.0), Level::Warning).await;
                }
            }
            msg.channel_id.say(&ctx.http, "Purged channel").await?;
        }    
        let _result = msg.delete(&ctx.http).await;
        
    }
    else {
        LOGGER.lock().await.write_log(format!("Failed to purge channel {}", msg.channel_id.0), Level::Debug).await;
    }
    Ok(())
}