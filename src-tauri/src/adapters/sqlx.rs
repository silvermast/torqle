use std::{collections::HashMap};
use mysql::{prelude::Queryable};
use mysql::Row;
use serde_json;

use super::{Adapter, AdapterOpts, QueryResult, QueryError};

#[derive(Clone, Debug)]
pub struct Mysql {
    pool: sqlx::AnyPool,
}

impl Adapter for Mysql {
    fn connect(opts: AdapterOpts) -> Result<Self, QueryError> where Self: Sized {
        let conn_uri = format!("mysql://{}:{}@{}:{}/", opts.user, opts.password, opts.host, opts.port);
        println!("Connecting to {}", conn_uri);
        let mysql_opts = mysql::Opts::from_url(conn_uri.as_str())
            .map_err(QueryError::from)?;
        
        let pool = mysql::Pool::new(mysql_opts).map_err(QueryError::from)?;
        println!("Created MySQL pool");

        Ok(Mysql { pool: pool })
    }

    fn query(&self, query: String, database: Option<String>) -> Result<QueryResult, QueryError> {
        let mut conn = self.pool.get_conn().map_err(|why| QueryError { error: why.to_string() })?;

        match database {
            Some(db_name) => {
                conn.query_drop(format!("USE {}", db_name)).map_err(|err| QueryError { error: err.to_string() })?;
                ()
            }
            None => ()
        }

        let start_time = std::time::SystemTime::now();
        println!("QUERY: {}", query);
        // let results = conn.query_map(query, |mut row: Row| {
        //     let mut row_map = HashMap::new();
        //     let columns = row.columns();
        //     for index in 0..columns.len() {
        //         let field: String = columns[index].name_str().into();
        //         let value: String = row.take(index).unwrap_or("".to_string());
        //         row_map.insert(field, value);
        //     }
        //     row_map
        // }).map_err(|why| QueryError { error: why.to_string() })?;

        let results = sqlx::query!(query).await.map_err(|why| QueryError::new("Failed to execute query"))?;
        println!("Returning query results");

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