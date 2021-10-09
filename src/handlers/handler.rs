use serenity::{
    async_trait,
    model::{
        guild::Member,
        channel::{GuildChannel, Message, ReactionType, Reaction},
        event::ResumedEvent,
        gateway::Ready,
        id::{MessageId, RoleId}
        //id::EmojiId,
    },
    prelude::*,
};
use crate::global::shared::{LOGGER, BOT_DATABASE};
use crate::logging::Level;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        LOGGER.lock().unwrap().write_log(format!("Connected as {}", ready.user.name), Level::Info);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        LOGGER.lock().unwrap().write_log(format!("Resumed!"), Level::Info);
    }
    
    //welp we will do it in appropritate way later
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        let msg_id = add_reaction.message_id.0;
        let mut role_id = String::new();
        if let ReactionType::Custom{animated:_animation, id: emoji_id, name: _name} = add_reaction.emoji {
            BOT_DATABASE.lock().unwrap().get_role_id(msg_id,emoji_id.0,&mut role_id);
        }
        else {
            return
        }
        let role = role_id.parse::<u64>().unwrap();
        if let Some(guild_id) = add_reaction.guild_id {
            if let Some(guild) = guild_id.to_guild_cached(&ctx).await {
                match add_reaction.user_id {
                    Some(user_id) =>  {
                        if let Ok(mut member) = guild.member(&ctx, &user_id).await {
                        match member.add_role(&ctx, RoleId(role)).await {
                            Ok(_) => {
                                println!("Added basic member role");
                            }
                            Err(err) => {
                                println!("Error ocurred: {:#?}", err);
                            }
                        };
                    }
                    }
                    None => {
                        println!("No user id!");
                    }
                }
            }
        } 
    }
}