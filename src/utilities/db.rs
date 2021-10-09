use rusqlite::{Connection};
use crate::global::shared::LOGGER;

pub struct Client {
    path: String,
    connection: Option<Connection>
}

impl Client {
    pub fn new() -> Client {
        Client {path: String::new(), connection: None}
    }
    
    pub fn configure(&mut self, path: &str) {
        self.path = String::from(path);
        self.connection = Some(Connection::open(path).expect("Error connecting to database"));
    }
    
    pub fn get_role_id(&mut self, msg_id: u64, emoji_id: &String, result: &mut String) {
        let mut query = self.connection
            .as_ref().unwrap()
            .prepare("SELECT role_id FROM roles_reactions WHERE msg_id = :msg_id AND emoji_id = :emoji_id").expect("Error");
        let mut rows = query.query(&[(":msg_id", msg_id.to_string().as_str()),(":emoji_id", 
            emoji_id.as_str())])
            .expect("Error");
        *result = String::from("No role");
        while let Some(row) = rows.next().expect("Error") {
            *result = row.get(0)
                .expect("Error");
        }    
    }
    
    pub fn add_reaction_role(&mut self, msg_id: String, emoji_id: String, role_id: String) {
        let mut query = self.connection
            .as_ref().unwrap()
            .prepare("INSERT INTO roles_reactions (msg_id, emoji_id, role_id) VALUES (:msg_id, :emoji_id, :role_id)").expect("Error");
        query.insert(&[(":msg_id", msg_id.as_str()),
            (":emoji_id", emoji_id.as_str()),
            (":role_id", role_id.as_str())
            ]).expect("Error");    
    }
}