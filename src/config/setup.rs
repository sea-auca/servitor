use crate::frameworks::framework;
use crate::handlers::handler;
use serde::Deserialize;
use serenity::{
    client::bridge::gateway::GatewayIntents,
    framework::standard::{macros::hook, CommandGroup, CommandResult, StandardFramework},
    model::channel::Message,
    prelude::*,
};
use std::fs;
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
    pub intents: GatewayIntents,
    pub framework: StandardFramework,
    pub handler: handler::Handler,
}

impl Settings {
    pub fn create_settings(path: String, group: &Vec<&'static CommandGroup>) -> Settings {
        let config = Config::from_toml(path);
        let mut framework = framework::create_framework("~");
        for g in group {
            framework.group_add(g);
        }
        let intents = GatewayIntents::all();
        let handler = handler::Handler;
        Settings {
            config,
            intents,
            framework,
            handler,
        }
    }
}