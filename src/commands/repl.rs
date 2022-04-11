use crate::global::shared::BOT_DATABASE;
use serenity::framework::standard::{macros::command, macros::group, CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::http::AttachmentType;
use std::process::Command;
use std::path::Path;
use std::fs::File;
use std::io::Write;
#[group]
#[owners_only]
#[commands(cxx_repl)]
struct Repl;


#[command]
#[description("Compile and run simple code in C. Source code should be in body of message.")]
#[min_args(1)]
#[usage("<source code>")]
async fn cxx_repl(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let code = args.message();
    let id = msg.id.0.to_string();
    let source = format!("{}.cpp",id);
    let mut file = File::create(&source)?;
    file.write_all(code.as_bytes())?;
    let output = Command::new("g++")
        .args(["-o", id.as_str(), source.as_str()])
        .output()?;
    if !(output.status.success()) {
        msg.channel_id.say(&ctx.http, "Compilation error").await?;
    }
    else {
        let output = Command::new(format!("./{}", id.as_str()))
            .output()?;
        if output.stdout.len() > 0 {
            msg.channel_id.say(&ctx.http, String::from_utf8(output.stdout).unwrap_or_else(|_e|{String::from("Cannot sent result")})).await?;
        } else if output.stderr.len() > 0 {
            msg.channel_id.say(&ctx.http, "Runtime error").await?;
        } else {
            msg.channel_id.say(&ctx.http, "Void output").await?;
        }   
    }    
    let _cleaner = Command::new("rm")
        .args(["-f", id.as_str(), source.as_str()])
        .output()?;
    Ok(())
}