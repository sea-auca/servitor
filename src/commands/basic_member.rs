use serenity::framework::standard::{macros::command, macros::group, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::process::Command;

#[group]
#[commands(make_me_a_member)]
struct BasicMember;

#[command]
async fn make_me_a_member(ctx: &Context, msg: &Message) -> CommandResult {
    println!("made member");
    Ok(())
}
