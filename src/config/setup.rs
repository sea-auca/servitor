//! Settings for bot to be launched with.
use crate::frameworks::framework;
use crate::global::shared::{BOT_DATABASE, LOGGER};
use crate::handlers::handler;
use envy;
use serde::Deserialize;
use serenity::{
    client::bridge::gateway::GatewayIntents,
    framework::standard::{CommandGroup, HelpCommand, StandardFramework},
};


/// The config is utility struct that helps to read and initialize configuration data for the bot.
/// The config stores environment specific data: bot token, database host address, database name, database username, database password and path to logfile.
/// Data for config is read from the enviroment variables prefixed with `DISCORD_`
/// The config is intended to be used as part of [`Settings`]. 
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
    /// Creates a new config. The data is being read from enviromental variables prefixed by `DISCORD_` 
    /// and deserialized via [`serde::Deserialize`] trait.
    /// 
    /// # Panics
    /// 
    /// Panics if at least one of enviromental variables is missing.
    pub fn new() -> Config {
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
    
    /// Returns configured bot token
    pub fn get_token(&self) -> String {
        String::from(&self.token)
    }
    
    /// Returns database configuration string in the format: 
    /// 
    /// `host = <database host address> user = <database username> password = <database password> dbname = <database name>`
    /// 
    /// This format is chosen since it is used by [`utilities::db::DatabaseClient`](crate::utilities::db::DatabaseClient) to establish connection with database.
    pub fn get_database_conf(&self) -> String {
        format!("host = {} user = {} password = {} dbname = {}", 
            &self.database_host, &self.database_user, &self.database_password, &self.database_name)
    }
    
    /// Returns configured path to logfile
    pub fn get_logfile(&self) -> String {
        String::from(&self.logfile)
    }
}


/// The settings is utility struct that stores the parameters bot should be started with
/// The settings includes: [`Config`], [`GatewayIntents`], [`StandardFramework`] and [`handler::Handler`]
/// It is intended that the settings fields are accessed at the startup process
/// to provide a correct initialization of bot's components.
pub struct Settings {
    pub config: Config,
    pub intents: GatewayIntents,
    pub framework: StandardFramework,
    pub handler: handler::Handler,
}

impl Settings {
    /// Creates new settings adding elements of `group` to `self.framework`'s command groups
    /// and setting `help` as `self.framework`'s help command. 
    /// Also configures [`shared::LOGGER`](struct@crate::global::shared::LOGGER) and [`shared::BOT_DATABASE`](struct@crate::global::shared::BOT_DATABASE).
    pub async fn create_settings(
        group: &Vec<&'static CommandGroup>,
        help: &'static HelpCommand,
    ) -> Settings {
        let config = Config::new();
        let mut framework = framework::create_framework("~").help(help);
        for g in group {
            framework.group_add(g);
        }
        {
            LOGGER
                .lock()
                .await
                .configure_logger(&config.get_logfile())
                .await;
            BOT_DATABASE
                .lock()
                .await
                .configure(&config.get_database_conf()).await;
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
