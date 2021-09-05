use serenity::{
    framework::standard::{macros::hook, CommandGroup, CommandResult, StandardFramework},
    model::{
        channel::{Message, ReactionType},
        id::EmojiId,
    },
    client::Context
};

pub fn create_framework(prefix: &str) -> StandardFramework {
        StandardFramework::new()
            .configure(|c| c.prefix(prefix))
            .before(before)
            .after(after)
            .normal_message(normal_message)
}

#[hook]
async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
   // if msg.author.bot {
   //     return false;
   // }
    println!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );
    true
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
async fn normal_message(ctx: &Context, msg: &Message) {
    println!("Received message {:?} from user {:?}", msg.content, msg.author.discriminator);
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


