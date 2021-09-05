use serenity::framework::standard::{macros::command, macros::group, CommandResult};
use serenity::model::{id::RoleId, prelude::*};
use serenity::prelude::*;

#[group]
#[commands(make_me_a_member)]
struct BasicMember;

#[command]
async fn make_me_a_member(ctx: &Context, msg: &Message) -> CommandResult {
    let member_role_id: RoleId = RoleId(884006016767889419);
    println!("made member {:#?}", msg);
    Ok(())
}
