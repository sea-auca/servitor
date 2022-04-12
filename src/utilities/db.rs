//! Client for connecting and interacting with database.
use crate::global::shared::LOGGER;
use crate::logging::Level;
use tokio_postgres::{Client, NoTls};

/// A wrapper struct around [`tokio_postgres::Client`].
/// 
/// This is used to establish connection with a PostgreSQL database and query bot's data.  
/// It is intended that struct is used as singleton in [`global::shared`](crate::global::shared)
/// and configured at startup within [`config::setup::Settings`](crate::config::setup::Settings), 
/// however, it is not necessary to do so. 
/// One should note, though, that current version of bot requires existence of configured instance of database client 
/// in [`global::shared`](crate::global::shared). 
pub struct DatabaseClient {
    params: String,
   // connection: Option<Connection<tokio_postgres::Socket, tokio_postgres::tls::NoTlsStream>>,
    client: Option<Client>
}

impl DatabaseClient {
    
    /// Creates a new database client. 
    /// [`Self::configure`] should be called before client can be used.
    pub fn new() -> DatabaseClient {
        DatabaseClient {
            params: String::new(),
           // connection: None,
            client: None
        }
    }
    
    /// Configures `self` and establishes connection with database. 
    /// `params` should be of the format: 
    /// 
    /// `host = <database host address> user = <database username> password = <database password> dbname = <database name>`
    /// 
    /// After configure is called client is ready to be used. 
    /// 
    /// # Panics
    /// 
    /// This function panics if connection to the PostgreSQL server failed due to some reason
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
