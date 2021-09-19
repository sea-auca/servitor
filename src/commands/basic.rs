use serenity::framework::standard::{macros::command, macros::group, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::process::Command;
const ABOUT_MSG: &str = "Hello! This is simple utility bot developed by our community. 
    We are still in process of development and new features will be added later";

#[group]
#[commands(ping, echo, fortune, about, member_me)]
struct General;

#[command]
#[max_args(0)]
#[description("Check whether bot is running and responding")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[min_args(1)]
#[description("Returns arguments provided to function.")]
#[usage("[args]")]
async fn echo(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, &msg.content).await?;
    Ok(())
}

#[command]
#[max_args(0)]
#[description("Generate nice quote.")]
#[usage("")]
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
#[description("Get information about bot.")]
#[num_args(0)]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, ABOUT_MSG).await?;
    Ok(())
}

#[command]
#[description("Grants user with member role")]
#[num_args(0)]
async fn member_me(ctx: &Context, msg: &Message) -> CommandResult {
    let member_role_id: RoleId = RoleId(884006016767889419);
    if let Some(guild_id) = msg.guild_id {
        if let Some(guild) = guild_id.to_guild_cached(&ctx).await {
            if let Ok(mut member) = guild.member(&ctx, &msg.author.id).await {
                match member.add_role(&ctx, &member_role_id).await {
                    Ok(_) => {
                        println!("Added basic member role");
                    }
                    Err(err) => {
                        println!("Error ocurred: {:#?}", err);
                    }
                };
            }
        }
    }
    Ok(())
}

