use rusqlite::{Connection};

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
    
    pub fn test_query(&mut self) -> String {
        let mut statement = self.connection.as_ref().unwrap().prepare("SELECT bar FROM foo").expect("Error");
        let mut rows = statement.query([]).expect("Error");
        let mut result = String::new();
        while let Some(r) = rows.next().expect("Error") {
            result = r.get(0).expect("Error");
        }
        result
    }
}