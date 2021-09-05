use serde::Deserialize;
use serenity::{
    framework::standard::{macros::hook, CommandGroup, CommandResult, StandardFramework},
    model::channel::Message,
    client::bridge::gateway::GatewayIntents,
    prelude::*,
};
use std::{fs};
use toml;
use crate::handlers::handler;
use crate::frameworks::framework;

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
    pub intents: GatewayIntents,
    pub framework: StandardFramework,
    pub handler: handler::Handler,
}

impl Settings {
    pub fn create_settings(path: String, group: &'static CommandGroup) -> Settings {
        let config = Config::from_toml(path);
        let mut framework = framework::create_framework("~");
        framework.group_add(group);
        let intents = GatewayIntents::all();
        let handler = handler::Handler;
        Settings { config, intents, framework, handler }
    }
}
