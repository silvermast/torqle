use std::{collections::HashMap, error::Error};
use mysql::prelude::Queryable;
use mysql::Row;

use super::{ClientConnection, QueryResult, QueryError};

#[derive(Clone, Debug)]
pub struct Sqlite {
    opts: HashMap<String, String>,
    pool: mysql::Pool,
}

impl ClientConnection for Sqlite {
    fn connect(opts: HashMap<String, String>) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let connection_opts = mysql::OptsBuilder::new().from_hash_map(&opts)?;
        let pool = mysql::Pool::new(connection_opts)?;

        Ok(Mysql { pool: pool, opts: opts })
    }

    fn test(opts: HashMap<String, String>) -> Result<bool, Box<dyn Error>> where Self: Sized {
        let connection_opts = mysql::OptsBuilder::new().from_hash_map(&opts)?;
        let pool = mysql::Pool::new(connection_opts)?;
        Ok(true)
    }

    fn change_schema(&mut self, value: String) -> Result<bool, Box<dyn Error>> {
        self.set_opt("db_name".to_string(), value)
    }

    fn set_opt(&mut self, option: String, value: String) -> Result<bool, Box<dyn Error>> {
        self.opts.insert(option, value);
        Ok(true)
    }

    fn get_opt(&self, option: String) -> Option<String> {
        match self.opts.get(&option.to_string()) {
            Some(value) => Some(value.to_string()),
            _ => None,
        }
    }

    fn query(&self, query: String) -> Result<QueryResult, QueryError> {
        let mut conn = self.pool.get_conn().map_err(|why| QueryError { error: why.to_string() })?;
        let mut query_actual: String;

        match self.get_opt("db_name".to_string()) {
            Some(db_name) => {
                // run a USE `database` query
                println!("QUERY: USE {}", db_name);
                conn.query_drop(format!("USE {}", db_name)).map_err(|err| QueryError { error: err.to_string() })?;
                true
            },
            _ => false,
        };
        
        let start_time = std::time::SystemTime::now();
        println!("QUERY: {}", query);
        let results = conn.query_map(query, |mut row: Row| {
            let mut row_map = HashMap::new();
            let columns = row.columns();
            for index in 0..columns.len() {
                let field: String = columns[index].name_str().into();
                let value: String = row.take(index).unwrap_or("".to_string());
                row_map.insert(field, value);
            }
            row_map
        }).map_err(|why| QueryError { error: why.to_string() })?;

        Ok(QueryResult {
            elapsed_ms: start_time.elapsed().expect("Error parsing elapsed timestamp!").as_millis().to_string(),
            num_rows: results.len().to_string(),
            rows: results,
        })
    }
    fn disconnect(&mut self) -> bool {
        true
    }
}