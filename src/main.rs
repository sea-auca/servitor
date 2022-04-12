mod commands;
mod config;
mod frameworks;
mod global;
mod handlers;
mod utilities;

use commands::basic::GENERAL_GROUP;
use commands::sudo::SUDO_GROUP;
use commands::repl::REPL_GROUP;
use commands::HELP;
use config::setup;
use global::shared::LOGGER;
use serenity::prelude::*;
use utilities::logging;
use std::env;

const USAGE_MESSAGE: &str = "Usage:
    \n\tservitor <SUBCOMMAND>
    \n\tSUBCOMMANDS:
    \n\t\thelp:\t Display this message and leave
    \n\t\tconfig <FILEPATH>:\t Set config file of bot to FILEPATH. Defaults to \"data/Config.toml\"
    \n\t\tRunning without argument is equivalent to 'servitor config data/Config.toml";


#[tokio::main]
async fn main() { 
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length == 2 && args[1] == "help" {
            println!("{}", USAGE_MESSAGE);
            return;
    }    
    let settings = setup::Settings::create_settings(
        &vec![&GENERAL_GROUP, &SUDO_GROUP, &REPL_GROUP],
        &HELP,
    ).await;
    let mut client = Client::builder(settings.config.get_token())
        .intents(settings.intents)
        .framework(settings.framework)
        .event_handler(settings.handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        LOGGER
            .lock()
            .unwrap()
            .write_log(format!("Client error: {:?}\n", why), logging::Level::Error);
    }
}
