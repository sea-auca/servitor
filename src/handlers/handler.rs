use crate::global::shared::{BOT_DATABASE, LOGGER};
use crate::logging::Level;
use serenity::{
    async_trait,
    model::{
        channel::{GuildChannel, Message, Reaction, ReactionType},
        event::ResumedEvent,
        gateway::Ready,
        guild::Member,
        id::{MessageId, RoleId, GuildId, ChannelId}, //id::EmojiId,
    },
    utils::MessageBuilder,
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        LOGGER
            .lock()
            .unwrap()
            .write_log(format!("Connected as {}", ready.user.name), Level::Info);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        LOGGER
            .lock()
            .unwrap()
            .write_log(format!("Resumed!"), Level::Info);
    }
    
    async fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, new_member: Member) {
        let general_channel_id = ChannelId(882988311138959430);
        let greeting = MessageBuilder::new()
            .push("Hello, ")
            .mention(&new_member)
            .push("! Please familiarize yourself with #rules and #announcements")
            .build();
        if let Err(why) = general_channel_id.say(&ctx.http, &greeting).await {
            LOGGER
                .lock()
                .unwrap()
                .write_log(format!("Error: {}", why), Level::Warning);
            return
        }
        LOGGER
            .lock()
            .unwrap()
            .write_log(format!("Send greetings to user {}", new_member.user.name), Level::Trace);    
        
    }

    //welp we will do it in appropritate way later
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        let msg_id = add_reaction.message_id.0;
        let mut role_id = String::new();
        if let ReactionType::Custom {
            animated: _animation,
            id: emoji_id,
            name: _name,
        } = add_reaction.emoji
        {
            BOT_DATABASE.lock().await.get_role_id(
                msg_id,
                &(emoji_id.0.to_string()),
                &mut role_id,
            ).await;
        } else if let ReactionType::Unicode(foo) = add_reaction.emoji {
            BOT_DATABASE
                .lock()
                .await
                .get_role_id(msg_id, &foo, &mut role_id).await;
        }
        if role_id == "No role" {
            return;
        }
        let role = role_id.parse::<u64>().unwrap();
        if let Some(guild_id) = add_reaction.guild_id {
            if let Some(guild) = guild_id.to_guild_cached(&ctx).await {
                match add_reaction.user_id {
                    Some(user_id) => {
                        if let Ok(mut member) = guild.member(&ctx, &user_id).await {
                            match member.add_role(&ctx, RoleId(role)).await {
                                Ok(_) => {
                                    LOGGER.lock().unwrap().write_log(
                                        format!("Given role {} to user {}", role, member.user.name),
                                        Level::Trace,
                                    );
                                }
                                Err(_) => {
                                    LOGGER.lock().unwrap().write_log(
                                        format!(
                                            "Error giving role {} to user {}",
                                            role, member.user.name
                                        ),
                                        Level::Warning,
                                    );
                                }
                            };
                        }
                    }
                    None => {
                        LOGGER
                            .lock()
                            .unwrap()
                            .write_log(format!("No user id provided in reaction"), Level::Trace);
                    }
                }
            }
        }
    }
}
