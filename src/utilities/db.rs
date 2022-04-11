use crate::global::shared::LOGGER;
use crate::logging::Level;
use postgres::{Client, NoTls, fallible_iterator::FallibleIterator};
use rusqlite::Connection;


pub struct DatabaseClient {
    params: String,
    connection: Option<Client>,
}

impl DatabaseClient {
    pub fn new() -> DatabaseClient {
        DatabaseClient {
            params: String::new(),
            connection: None,
        }
    }

    pub fn configure(&mut self, params: &str) {
        self.params = String::from(params);
        //we can use unwrap here since if connection to database failed bot itself is a failure
        self.connection = Some(Client::connect(params, NoTls).unwrap());
    }

    pub fn get_role_id(&mut self, msg_id: u64, emoji_id: &String, result: &mut String) {
        *result = String::from("No role");
        let client = self.connection.as_mut().unwrap();
        
        let executed = client.query_opt(
            "SELECT role_id FROM role_reactions WHERE msg id = $1 AND emoji_id = $2", 
            &[&msg_id.to_string().as_str(), &emoji_id.as_str()]);
        match executed {
            Ok(row) => {
                match row {
                    Some(row) => {
                        *result = row.get("role_id");
                    }
                    None => {
                        return
                    }
                }    
            }
            Err(err) => {
                LOGGER
                    .lock()
                    .unwrap()
                    .write_log(format!("Error executing select statement: {}", err), Level::Error);
            }   
        }
    }

    pub fn add_reaction_role(&mut self, msg_id: String, emoji_id: String, role_id: String) {
        let client = self.connection.as_mut().unwrap();
        let executed = client.execute(
            "INSERT INTO roles_reactions (msg_id, emoji_id, role_id) VALUES ($1, $2, $3)", 
            &[&msg_id.as_str(), &emoji_id.as_str(), &role_id.as_str()]);
        match executed {
            Ok(count) => {
                LOGGER
                    .lock()
                    .unwrap()
                    .write_log(format!("Inserted {} rows", count), Level::Debug);    
            }
            Err(err) => {
                LOGGER
                    .lock()
                    .unwrap()
                    .write_log(format!("Error executing insert statement: {}", err), Level::Error);
            }   
        }
        
        /*let query = self.connection
            .as_ref().unwrap()
            .prepare("INSERT INTO roles_reactions (msg_id, emoji_id, role_id) VALUES ($1, $2, $3)");
        match query {
            Ok(mut query) => {
                if let Err(_) = query.insert(&[
                    (":msg_id", msg_id.as_str()),
                    (":emoji_id", emoji_id.as_str()),
                    (":role_id", role_id.as_str()),
                ]) {
                    LOGGER
                        .lock()
                        .unwrap()
                        .write_log(format!("Error adding role to database"), Level::Error)
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
        }*/
    }
}
