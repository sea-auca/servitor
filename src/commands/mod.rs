pub mod basic;
pub mod sudo;

use serenity::{framework::standard::{Args, macros::help, help_commands, CommandResult, CommandGroup, HelpOptions}, model::prelude::*, prelude::*};

use std::collections::HashSet;

#[help]
#[individual_command_tip = "'~help <command name> to see information about specific command'"]
#[command_not_found_text = "Sorry, we currently do not have such command!"]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
async fn help(context: &Context, msg: &Message, args: Args, help_options: &'static HelpOptions, groups: &[&'static CommandGroup], owners: HashSet<UserId>) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
