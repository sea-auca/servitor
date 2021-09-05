use crate::frameworks::framework;
use crate::handlers::handler;
use serde::Deserialize;
use serenity::{
    client::bridge::gateway::GatewayIntents,
    framework::standard::{HelpCommand, CommandGroup, StandardFramework}
};
use std::fs;
use toml;
use crate::global::shared;

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
    pub fn create_settings(path: String, group: &Vec<&'static CommandGroup>, help: &'static HelpCommand) -> Settings {
        let config = Config::from_toml(path);
        let mut framework = framework::create_framework("~").help(help);
        for g in group {
            framework.group_add(g);
        }
        {
            (*shared::LOGGER).lock().unwrap().configure_logger("logs/bot.log");
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
