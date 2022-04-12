use crate::global::shared::LOGGER;
use crate::logging::Level;
use tokio_postgres::{Client, Connection, NoTls};


pub struct DatabaseClient {
    params: String,
   // connection: Option<Connection<tokio_postgres::Socket, tokio_postgres::tls::NoTlsStream>>,
    client: Option<Client>
}

impl DatabaseClient {
    pub fn new() -> DatabaseClient {
        DatabaseClient {
            params: String::new(),
           // connection: None,
            client: None
        }
    }

    pub async fn configure(&mut self, params: &str) {
        let (client, connection) = tokio_postgres::connect(params, NoTls)
            .await.unwrap();
        
        self.params = String::from(params);
        self.client = Some(client);
        //self.connection = Some(connection);
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                panic!("connection error: {}", e);
            }
        });
    }

    pub async fn get_role_id(&mut self, msg_id: u64, emoji_id: &String, result: &mut String) {
        *result = String::from("No role");
        let client = self.client.as_mut().unwrap();
        
        let executed = client.query_opt(
            "SELECT role_id FROM roles_reactions WHERE msg_id = $1 AND emoji_id = $2", 
            &[&msg_id.to_string().as_str(), &emoji_id.as_str()]).await;
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

    pub async fn add_reaction_role(&mut self, msg_id: String, emoji_id: String, role_id: String) {
        let client = self.client.as_mut().unwrap();
        let executed = client.execute(
            "INSERT INTO roles_reactions (msg_id, emoji_id, role_id) VALUES ($1, $2, $3)", 
            &[&msg_id.as_str(), &emoji_id.as_str(), &role_id.as_str()]).await;
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
        
    }
}
