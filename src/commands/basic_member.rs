use serenity::framework::standard::{macros::command, macros::group, CommandResult};
use serenity::model::{guild::Guild, id::RoleId, prelude::*};
use serenity::prelude::*;

#[group]
#[commands(member_me)]
struct BasicMember;

#[command]
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
