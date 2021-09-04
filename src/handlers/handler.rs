use serenity::{
    async_trait,
    model::{
        channel::{Message, ReactionType},
        event::ResumedEvent,
        gateway::Ready,
        id::EmojiId,
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

    async fn message(&self, ctx: Context, msg: Message) {
        if &msg.content.chars().count() > &0 && &msg.content.chars().next().unwrap() == &'~' {
            return;
        }
        if msg.content.contains("Яман") || msg.content.contains("яман") {
            let reaction_type = ReactionType::Custom {
                animated: false,
                id: EmojiId(798454339134816256),
                name: Some(String::from(":shit_taster:")),
            };
            if let Err(why) = msg.react(&ctx, reaction_type).await {
                println!("Error reacting: {:?}", why);
            }
            if let Err(why) = msg.channel_id.say(&ctx.http, "Курлык-курлык!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
