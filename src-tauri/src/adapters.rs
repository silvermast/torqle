use std::collections::HashMap;
use std::panic::catch_unwind;

use chrono::{Utc, TimeZone};
use mysql_async::prelude::Queryable;
use mysql_async::{Row, OptsBuilder, Pool, Value};
// use mysql_async::Value;
use serde::{Serialize, ser::SerializeMap, Serializer};
use serde_json::{json, Number};
// use mysql_async::prelude::*;
// use mysql::Row;
use ssh_jumper::model::SshForwarderEnd;
use tokio::sync::oneshot::Receiver;

pub use serde_json::Value as JsonValue;
pub use serde_json::Map as JsonMap;

use crate::ssh;

/**
 * @see https://github.com/jasondeewright/sqlx
 */

 #[derive(Clone, Debug, serde::Deserialize, Copy)]
pub enum DriverType {
    MySQL,
    PostgreSQL,
    Sqlite,
    MongoDB,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct AdapterOpts {
    pub driver: DriverType,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}
// impl AdapterOpts {
//     fn uri(&self) -> String {
//         match self.driver {
//             DriverType::MySQL => format!("mysql://{}:{}@{}:{}/?prefer_socket=false", self.user, self.password, self.host, self.port),
//             DriverType::PostgreSQL => format!("postgres://{}:{}@{}:{}/", self.user, self.password, self.host, self.port),
//             DriverType::Sqlite => format!("sqlite://{}:{}@{}:{}/", self.user, self.password, self.host, self.port),
//             DriverType::MongoDB => format!("mongodb://{}:{}@{}:{}/", self.user, self.password, self.host, self.port),
//         }
//     }
// }

#[derive(Clone, Debug, serde::Deserialize)]
pub struct SshOpts {
    pub host: String,
    pub keyfile: Option<String>,
    pub password: String,
    pub port: u16,
    pub user: String,
}

#[derive(Serialize, Debug)]
pub struct QueryResult {
    pub elapsed_ms: String,
    pub num_rows: String,
    // fields: Vec<String>,
    pub fields: Vec<String>,
    pub rows: Vec<HashMap<String, JsonValue>>,
}

#[derive(Serialize, Debug)]
pub struct QueryError {
    // query: String,
    pub error: String,
}
impl QueryError {
    pub fn from<E: std::fmt::Display>(err: E) -> QueryError {
        let error_string = format!("{}", err);
        println!("Error: {}", error_string); // debug!
        QueryError { error: error_string }
    }
}
#[derive(Clone)]
pub struct Adapter {
    pool: Pool,
}
impl Adapter {
    pub async fn connect(opts: AdapterOpts, ssh_opts: Option<SshOpts>) -> Result<Self, QueryError> where Self: Sized {
        let (driver_opts, ssh_tunnel): (AdapterOpts, Option<Receiver<SshForwarderEnd>>) = match ssh_opts {
            Some(ssh_opts_actual) => {
                let (new_opts, ssh_tunnel) = ssh::tunnel(opts, ssh_opts_actual).await?;
                (new_opts, Some(ssh_tunnel))
            },
            None => (opts, None)
        };

        let mysql_opts = OptsBuilder::default()
            .ip_or_hostname(driver_opts.host)
            .tcp_port(driver_opts.port)
            .user(Some(driver_opts.user))
            .pass(Some(driver_opts.password))
            .prefer_socket(false);
        
        let pool = Pool::new(mysql_opts);

        Ok(Adapter { pool: pool })
    }
    
    pub async fn disconnect(&mut self) -> bool {
        true
    }

    pub async fn query(&self, query: String, database: Option<String>) -> Result<QueryResult, QueryError> {
        let mut conn = self.pool.get_conn().await.map_err(QueryError::from)?;

        if let Some(db_name) = database {
            conn.query_drop(format!("USE `{}`", db_name)).await.map_err(QueryError::from)?;
        }

        println!("QUERY: {}", query);
        let start_time = std::time::SystemTime::now();

        let query_results: Vec<Row> = conn.query(query.as_str()).await.map_err(QueryError::from)?;
        println!("Retrieved {} results", query_results.len());
        let mut results: Vec<HashMap<String, JsonValue>> = Vec::new();

        let fields: Vec<String> = if query_results.is_empty() {
            Vec::new()
        } else {
            Vec::from_iter(query_results[0].columns().iter().map(|c| c.name_str().to_string()))
        };

        for row in query_results {
            let row_map = parse_sqlx_row(row)?;
            results.push(row_map);
        }

        Ok(QueryResult {
            elapsed_ms: start_time.elapsed().expect("Error parsing elapsed timestamp!").as_millis().to_string(),
            num_rows: results.len().to_string(),
            rows: Vec::from_iter(results),
            fields: fields,
        })
    }
}

fn parse_sqlx_row(row: Row) -> Result<HashMap<String, JsonValue>, QueryError> {
    let mut map = HashMap::new();
    let columns = row.columns();

    for index in 0..columns.len() {
        let field: String = columns[index].name_str().into();

        let value: JsonValue = match row.get(index).unwrap() {
            Value::NULL => JsonValue::Null,
            Value::Bytes(x) => JsonValue::from(unsafe { String::from_utf8_unchecked(x) }),
            Value::Int(x) => JsonValue::from(x as i64),
            Value::UInt(x) => JsonValue::from(x as u64),
            Value::Float(x) => JsonValue::from(x as f32),
            Value::Double(x) => JsonValue::from(x as f64),
            Value::Date(year, mon, day, 0u8, 0u8, 0u8, 0u32) => JsonValue::from(format!("{}-{}-{}", year, mon, day)),
            Value::Date(year, mon, day, hour, min, sec, usec) => {
                // Chrono's LocalResult only implements unwrap with a panic. So we need to catch it.
                match catch_unwind(|| Utc.with_ymd_and_hms(year as i32, mon as u32, day as u32, hour as u32, min as u32, sec as u32).unwrap()) {
                    Ok(datetime) => JsonValue::String(datetime.to_rfc3339()),
                    Err(err) => JsonValue::from("Invalid DateTime"),
                }
            },
            Value::Time(neg, days, hours, mins, secs, usecs) => {
                // Chrono's LocalResult only implements unwrap with a panic. So we need to catch it.
                match catch_unwind(|| Utc.with_ymd_and_hms(0, 0, 0, hours as u32, mins as u32, secs as u32).unwrap()) {
                    Ok(datetime) => JsonValue::String(datetime.format("%T").to_string()),
                    Err(_) => JsonValue::from("Invalid Time"),
                }
            }
            _ => JsonValue::from("Unsupported type"),
        };
        map.insert(field, value);
    }

    Ok(map)
}
