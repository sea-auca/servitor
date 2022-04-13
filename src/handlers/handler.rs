//! Implementation of default handler for the bot.
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
            .await
            .write_log(format!("Connected as {}", ready.user.name), Level::Info).await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        LOGGER
            .lock()
            .await
            .write_log(format!("Resumed!"), Level::Info).await;
    }
    
    async fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, new_member: Member) {
        let general_channel_id = ChannelId(882988311138959430);
        let (rules_channel, announcements, roles_channel) = 
            (ChannelId(709439871512477756),ChannelId(896342671973572618),ChannelId(888679233948377109));
        let greeting = MessageBuilder::new()
            .push("Hello, ")
            .mention(&new_member)
            .push("! Please familiarize yourself with  ")
            .channel(rules_channel)
            .push(", ")
            .channel(announcements)
            .push(" and ")
            .channel(roles_channel)
            .build();
        if let Err(why) = general_channel_id.say(&ctx.http, &greeting).await {
            LOGGER
                .lock()
                .await
                .write_log(format!("Error: {}", why), Level::Warning).await;
            return
        }
        LOGGER
            .lock()
            .await
            .write_log(format!("Send greetings to user {}", new_member.user.name), Level::Trace).await;    
        
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
        let guild_id  = add_reaction.guild_id;
        if guild_id == None {
            return
        }
        let guild = guild_id
            .unwrap()
            .to_guild_cached(&ctx)
            .await;
        let guild = match guild {
            Some(guild) => {
                guild
            }
            None => {
                return
            }
        };
        if add_reaction.user_id == None {
            LOGGER
                .lock()
                .await
                .write_log(format!("No user id provided in reaction"), Level::Trace)
                .await;
            return    
        }
        let member = guild.member(&ctx, add_reaction.user_id.unwrap()).await;
        let mut member = match member {
            Ok(member) => {
                member
            }
            Err(_) => {
                return
            }
        };
        match member.add_role(&ctx, RoleId(role)).await {
            Ok(_) => {
                LOGGER.lock().await.write_log(
                    format!("Given role {} to user {}", role, member.user.name),
                    Level::Trace,
                ).await;
            }
            Err(_) => {
                LOGGER.lock().await.write_log(
                    format!(
                        "Error giving role {} to user {}",
                        role, member.user.name
                    ),
                    Level::Warning,
                ).await;
            }
        }
    }
}
