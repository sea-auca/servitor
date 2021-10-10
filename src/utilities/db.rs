use crate::global::shared::LOGGER;
use crate::logging::Level;
use rusqlite::Connection;

pub struct Client {
    path: String,
    connection: Option<Connection>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            path: String::new(),
            connection: None,
        }
    }

    pub fn configure(&mut self, path: &str) {
        self.path = String::from(path);
        self.connection = Some(Connection::open(path).expect("Error connecting to database"));
    }

    pub fn get_role_id(&mut self, msg_id: u64, emoji_id: &String, result: &mut String) {
        *result = String::from("No role");
        let query = self.connection.as_ref().unwrap().prepare(
            "SELECT role_id FROM roles_reactions WHERE msg_id = :msg_id AND emoji_id = :emoji_id",
        );
        LOGGER.lock().unwrap().write_log(format!("Prepared select statement"), Level::Debug);
        match query {
            Ok(mut query) => {
                let rows = query.query(&[
                    (":msg_id", msg_id.to_string().as_str()),
                    (":emoji_id", emoji_id.as_str()),
                ]);
                match rows {
                    Ok(mut rows) => {
                        while let Some(row) = rows.next().expect("Error") {
                            *result = row.get(0).unwrap_or_else(|_err| {
                                LOGGER
                                    .lock()
                                    .unwrap()
                                    .write_log(format!("Error reading row"), Level::Error);
                                String::from("No role")
                            });
                        }
                    }
                    Err(_) => {
                        LOGGER
                            .lock()
                            .unwrap()
                            .write_log(format!("Error performing select"), Level::Error);
                    }
                }
            }
            Err(_err) => {
                LOGGER
                    .lock()
                    .unwrap()
                    .write_log(format!("Error preparing select"), Level::Error);
            }
        }
    }

    pub fn add_reaction_role(&mut self, msg_id: String, emoji_id: String, role_id: String) {
        let query = self.connection
            .as_ref().unwrap()
            .prepare("INSERT INTO roles_reactions (msg_id, emoji_id, role_id) VALUES (:msg_id, :emoji_id, :role_id)");
        match query {
            Ok(mut query) => {
                if let Err(_) = query.insert(&[
                    (":msg_id", msg_id.as_str()),
                    (":emoji_id", emoji_id.as_str()),
                    (":role_id", role_id.as_str()),
                ]) {
                    LOGGER.lock().unwrap().write_log(
                        format!("Error adding role to database"),
                        Level::Error,
                    )
                } else {
                    LOGGER
                        .lock()
                        .unwrap()
                        .write_log(format!("Successfully added role to database"), Level::Debug)
                }
            }
            Err(_) => {
                LOGGER
                    .lock()
                    .unwrap()
                    .write_log(format!("Error prearing insert"), Level::Error);
            }
        }
    }
}
