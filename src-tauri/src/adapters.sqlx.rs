use chrono::{NaiveDateTime, FixedOffset};
use serde::{Serialize, ser::SerializeMap, Serializer};
use serde_json::{json, Number};
use sqlx::{AnyPool, MySqlPool, Row, Executor, Column, any::{AnyTypeInfo, AnyRow, AnyValueRef}, mysql::MySqlRow, types::Json, Database, Decode, ValueRef, TypeInfo, query_as_unchecked};
use sqlx::types::{chrono::{DateTime, Utc}, BigDecimal, };
use ssh_jumper::model::SshForwarderEnd;
use tokio::sync::oneshot::Receiver;

pub use serde_json::Value as JsonValue;
pub use serde_json::Map as JsonMap;

use crate::ssh;

/**
 * @see https://github.com/jasondeewright/sqlx
 */

 #[derive(Clone, Debug, serde::Deserialize)]
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
impl AdapterOpts {
    fn uri(&self) -> String {
        match self.driver {
            DriverType::MySQL => format!("mysql://{}:{}@{}:{}/", self.user, self.password, self.host, self.port),
            DriverType::PostgreSQL => format!("postgres://{}:{}@{}:{}/", self.user, self.password, self.host, self.port),
            DriverType::Sqlite => format!("sqlite://{}:{}@{}:{}/", self.user, self.password, self.host, self.port),
            DriverType::MongoDB => format!("mongodb://{}:{}@{}:{}/", self.user, self.password, self.host, self.port),
        }
    }
}

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
    pub rows: Vec<JsonMap<String, JsonValue>>,
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
    pool: AnyPool,
}
impl Adapter {
    pub async fn connect(opts: AdapterOpts, ssh_opts: Option<SshOpts>) -> Result<Self, QueryError> where Self: Sized {
        sqlx::any::install_default_drivers(); // needed in 0.7

        let (driver_opts, ssh_tunnel): (AdapterOpts, Option<Receiver<SshForwarderEnd>>) = match ssh_opts {
            Some(ssh_opts_actual) => {
                let (new_opts, ssh_tunnel) = ssh::tunnel(opts, ssh_opts_actual).await?;
                (new_opts, Some(ssh_tunnel))
            },
            None => (opts, None)
        };

        let pool = AnyPool::connect(driver_opts.uri().as_str()).await.map_err(QueryError::from)?;

        Ok(Adapter { 
            pool: pool,
        })
    }
    
    pub async fn disconnect(&mut self) -> bool {
        self.pool.close().await;
        true
    }

    pub async fn query(&self, query: String, database: Option<String>) -> Result<QueryResult, QueryError> {
        let mut conn = self.pool.acquire().await.map_err(QueryError::from)?;

        if let Some(db_name) = database {
            conn.execute(format!("USE {}", db_name).as_str()).await.map_err(QueryError::from)?;
        }

        println!("QUERY: {}", query);
        let start_time = std::time::SystemTime::now();
        conn.execute_many(query)
        let query_results = conn.fetch_all(query.as_str()).await.map_err(QueryError::from)?;
        println!("Retrieved {} results", query_results.len());
        let mut results: Vec<JsonMap<String, JsonValue>> = Vec::new();
        
        for row in query_results {
            let row_map = parse_sqlx_row(row)?;
            results.push(row_map);
        }

        Ok(QueryResult {
            elapsed_ms: start_time.elapsed().expect("Error parsing elapsed timestamp!").as_millis().to_string(),
            num_rows: results.len().to_string(),
            rows: Vec::from_iter(results),
        })
    }
}

fn parse_sqlx_row(row: AnyRow) -> Result<JsonMap<String, JsonValue>, QueryError> {
    let mut map = JsonMap::new();

    for column in row.columns() {
        let col = column.name();
        let value = serialize_anyvalueref(column, &row);
        // let value: String = row.try_get(column.ordinal()).unwrap_or("Unsupported field type!".to_string());
        map.insert(col.to_string(), value);
    }

    Ok(map)
}



fn serialize_anyvalueref<C: Column>(column: &C, row: &AnyRow) -> JsonValue {
    let type_info = column.type_info();
    let ord = column.ordinal();
    let raw = row.try_get_raw(ord).unwrap();

    println!("Type: {}, Col: {}", type_info.name(), column.name());

    match type_info.name() {
        "BOOLEAN" => {
            let val: bool = row.try_get(ord).unwrap_or_default();
            JsonValue::from(val)
        },
        "SMALLINT" => {
            let val: i64 = row.try_get(ord).unwrap_or_default();
            JsonValue::from(val)
        },
        "INTEGER" => {
            let val: i64 = row.try_get(ord).unwrap_or_default();
            JsonValue::from(val)
        },
        "BIGINT" => {
            let val: i64 = row.try_get(ord).unwrap_or_default();
            JsonValue::from(val)
        },
        "REAL" => {
            let val: f64 = row.try_get(ord).unwrap_or_default();
            JsonValue::from(val)
        },
        "DOUBLE" => {
            let val: f64 = row.try_get(ord).unwrap_or_default();
            JsonValue::from(val)
        },
        "TEXT" => {
            let val: String = row.try_get(ord).unwrap_or_default();
            JsonValue::from(val)
        },
        "BLOB" => {
            let val: String = row.try_get(ord).unwrap_or_default();
            JsonValue::from(val)
        },
        // "TIMESTAMP" => {
        //     let val: DateTime<FixedOffset> = row.try_get(ord).unwrap_or_default();
        //     JsonValue::from(val)
        // }
        "NULL" => JsonValue::Null,
        // AnyTypeInfoKind::Null => JsonValue::Null,
        // AnyTypeInfoKind.Bool => serde_json::from_slice(v),
        // AnyTypeInfoKind.SmallInt,
        // AnyTypeInfoKind.Integer,
        // AnyTypeInfoKind.BigInt,
        // AnyTypeInfoKind.Real,
        // AnyTypeInfoKind.Double,
        // AnyTypeInfoKind.Text,
        // AnyTypeInfoKind.Blob,
        _ => {
            println!("Unsupported type! {:?}", type_info.name());
            JsonValue::Null
        },
    }
}
