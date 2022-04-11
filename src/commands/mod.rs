pub mod basic;
pub mod sudo;
pub mod repl;

use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::prelude::*,
    prelude::*,
};

use std::collections::HashSet;

#[help]
#[individual_command_tip = "'~help <command name> to see information about specific command'"]
#[command_not_found_text = "Sorry, we currently do not have such command!"]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let mut msg_clone = msg.clone();
    let dm = msg_clone.author.create_dm_channel(&context.http).await;
    match dm {
        Ok(dm) => {
            msg_clone.channel_id = dm.id;
            let _ = help_commands::with_embeds(context, &msg_clone, args, help_options, groups, owners).await; 
        }
        Err(_) => {
            let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
        }
    }
    
    Ok(())
}
