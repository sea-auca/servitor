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

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
    
    
    //welp we will do it in appropritate way later
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        if add_reaction.message_id != MessageId(884019412376637442) {
            return
        }
    
        if let Some(guild_id) = add_reaction.guild_id {
            println!("Foo");
            if let Some(guild) = guild_id.to_guild_cached(&ctx).await {
                match add_reaction.user_id {
                    Some(user_id) =>  {
                        if let Ok(mut member) = guild.member(&ctx, &user_id).await {
                        match member.add_role(&ctx, RoleId(883626459980247060)).await {
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