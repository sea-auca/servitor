/*use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::{prelude::*, user::*, id, guild};
use serenity::prelude::*;

const SUDO_ROLE: RoleId = RoleId(704984252026191882);
#[command]
async fn disconnect(ctx: &Context, msg: &Message) -> CommandResult {

    let has_role = msg.author.has_role(&ctx.http, GuildContainer::Id(msg.guild_id.unwrap()), SUDO_ROLE).await?;
    if !has_role {
        Ok(())
    }
    else {
        msg.reply(ctx, "Shutting down");

        Ok(())
    }
    //to be implemented
}*/
