pub mod commands;
use commands::basic::GENERAL_GROUP;
use commands::repl::REPL_GROUP;
use commands::sudo::SUDO_GROUP;
use commands::HELP;

use crate::config::setup;
use crate::frameworks::framework;
use crate::handlers::handler;
use serenity::{
	client::bridge::gateway::GatewayIntents,
	client::Client,
	framework::standard::{CommandGroup, HelpCommand, StandardFramework},
	Result,
};

pub struct Server {
	client: Client,
}

impl Server {
	pub async fn create(settings: setup::Settings) -> Result<Server> {
		let client = Client::builder(settings.config.get_token())
			.intents(GatewayIntents::all())
			.framework(Server::create_framework(
				&vec![&GENERAL_GROUP, &SUDO_GROUP, &REPL_GROUP],
				&HELP,
			))
			.event_handler(handler::Handler)
			.await?;
		Ok(Server { client })
	}
	pub async fn start(&mut self) -> Result<()> {
		self.client.start().await
	}
	fn create_framework(
		group: &Vec<&'static CommandGroup>,
		help: &'static HelpCommand,
	) -> StandardFramework {
		let mut framework = framework::create_framework("~").help(help);
		for g in group {
			framework.group_add(g);
		}
		return framework;
	}
}
