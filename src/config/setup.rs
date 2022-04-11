use crate::frameworks::framework;
use crate::global::shared::{BOT_DATABASE, LOGGER};
use crate::handlers::handler;
use envy;
use serde::Deserialize;
use serenity::{
    client::bridge::gateway::GatewayIntents,
    framework::standard::{CommandGroup, HelpCommand, StandardFramework},
};



#[derive(Debug, Deserialize)]
pub struct Config {
    token: String,
    database_host: String,
    database_name: String,
    database_user: String,
    database_password: String,
    logfile: String
}

impl Config {
    fn new() -> Config {
        match envy::prefixed("DISCORD_").from_env::<Config>() {
            Ok(config) => {
                println!("{:?}", config);
                return config
            }
            Err(error) => {
                panic!("{:#}", error)
            }
            
        }
    }
    pub fn get_token(&self) -> String {
        String::from(&self.token)
    }
    pub fn get_database_conf(&self) -> String {
        format!("host = {} user = {} password = {} dbname = {}", 
            &self.database_host, &self.database_user, &self.database_password, &self.database_name)
    }
    pub fn get_logfile(&self) -> String {
        String::from(&self.logfile)
    }
}

pub struct Settings {
    pub config: Config,
    pub intents: GatewayIntents,
    pub framework: StandardFramework,
    pub handler: handler::Handler,
}

impl Settings {
    pub fn create_settings(
        group: &Vec<&'static CommandGroup>,
        help: &'static HelpCommand,
    ) -> Settings {
        let config = Config::new();
        let mut framework = framework::create_framework("~").help(help);
        for g in group {
            framework.group_add(g);
        }
        {
            LOGGER.lock().unwrap().configure_logger(&config.get_logfile());
            BOT_DATABASE
                .lock()
                .unwrap()
                .configure(&config.get_database_conf());
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
