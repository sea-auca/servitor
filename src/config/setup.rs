use serde::Deserialize;
use serenity::{
    client::bridge::gateway::GatewayIntents,
    framework::standard::{macros::hook, CommandGroup, CommandResult, StandardFramework},
    model::channel::Message,
    prelude::*,
};
use std::{error::Error, fs};
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    token: String,
}

impl Config {
    fn from_toml(path: String) -> Config {
        let contents = fs::read_to_string(path).expect("Error opening and reading file");
        let config: Config = toml::from_str(contents.as_str()).expect("Error parsing config");
        println!("{:?}", config);
        config
    }
    pub fn get_token(&self) -> &String {
        &self.token
    }
}

pub struct Settings {
    pub config: Config,
    pub framework: StandardFramework,
}

impl Settings {
    pub fn create_settings(path: String, group: &'static CommandGroup) -> Settings {
        let config = Config::from_toml(path);
        let framework = StandardFramework::new()
            .configure(|c| c.prefix("~"))
            .before(before)
            .after(after)
            .group(group);
        Settings { config, framework }
    }
}
#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
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
