use std::collections::HashMap;
use std::error::Error;
use serde::Serialize;

pub mod mysql;

pub enum Drivers {
    Mysql,
    Postgres,
    Mongodb,
    Sqlite,
}

#[derive(Serialize, Debug)]
pub struct QueryResult {
    pub elapsed_ms: String,
    pub num_rows: String,
    // fields: Vec<String>,
    pub rows: Vec<HashMap<String, String>>,
}

#[derive(Serialize, Debug)]
pub struct QueryError {
    // query: String,
    pub error: String,
}


pub trait ClientConnection {
    fn connect(opts: HashMap<String, String>) -> Result<Self, Box<dyn Error>> where Self: Sized;
    fn test(opts: HashMap<String, String>) -> Result<bool, Box<dyn Error>> where Self: Sized;
    // fn connect() -> Result<bool, String>;
    fn disconnect(&mut self) -> bool;
    fn query(&self, query: String) -> Result<QueryResult, QueryError>;
    fn change_schema(&mut self, value: String) -> Result<bool, Box<dyn Error>>;
    fn get_opt(&self, option: String) -> Option<String>;
    fn set_opt(&mut self, option: String, value: String) -> Result<bool, Box<dyn Error>> where Self: Sized;
}

// struct MongodbConnection {}
// struct SqliteConnection {}
// struct PostgresConnection {}
