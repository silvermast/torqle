use std::collections::HashMap;
use std::panic::catch_unwind;

use chrono::{TimeZone, Utc};
use mysql_async::prelude::Queryable;
use mysql_async::{OptsBuilder, Pool, Row, Value};

pub use serde_json::Value as JsonValue;

use super::{Adapter, AdapterOpts, QueryResult};
use crate::AppError;

pub async fn connect(opts: AdapterOpts) -> Result<MySQLAdapter, AppError>
where
    MySQLAdapter: Sized,
{
    let mysql_opts = OptsBuilder::default()
        .ip_or_hostname(opts.host)
        .tcp_port(opts.port as u16)
        .user(Some(opts.user))
        .pass(Some(opts.password))
        .prefer_socket(false);

    let pool = Pool::new(mysql_opts);

    // attempt a connection in order to validate credentials.
    pool.get_conn().await.map_err(AppError::from)?;

    Ok(MySQLAdapter { pool: pool })
}

#[derive(Clone)]
pub struct MySQLAdapter {
    pool: Pool,
}
impl Adapter for MySQLAdapter {
    async fn query(
        &self,
        query: String,
        database: Option<String>,
    ) -> Result<QueryResult, AppError> {
        let mut conn = self.pool.get_conn().await.map_err(AppError::from)?;

        if let Some(db_name) = database {
            conn.query_drop(format!("USE `{}`", db_name))
                .await
                .map_err(AppError::from)?;
        }

        println!("QUERY: {}", query);
        let start_time = std::time::SystemTime::now();

        let query_results: Vec<Row> = conn.query(query.as_str()).await.map_err(AppError::from)?;
        println!("Retrieved {} results", query_results.len());
        let mut results: Vec<HashMap<String, JsonValue>> = Vec::new();

        for row in query_results {
            let row_map = parse_row(row)?;
            results.push(row_map);
        }

        Ok(QueryResult::make(start_time, Vec::from_iter(results)))
    }

    async fn disconnect(&mut self) -> Result<bool, AppError> {
        match self.to_owned().pool.disconnect().await {
            Ok(_) => Ok(true),
            Err(why) => Err(AppError::from(why)),
        }
    }
}

fn parse_row(row: Row) -> Result<HashMap<String, JsonValue>, AppError> {
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
            Value::Date(year, mon, day, 0u8, 0u8, 0u8, 0u32) => {
                JsonValue::from(format!("{}-{}-{}", year, mon, day))
            }
            Value::Date(year, mon, day, hour, min, sec, _usec) => {
                // Chrono's LocalResult only implements unwrap with a panic. So we need to catch it.
                match catch_unwind(|| {
                    Utc.with_ymd_and_hms(
                        year as i32,
                        mon as u32,
                        day as u32,
                        hour as u32,
                        min as u32,
                        sec as u32,
                    )
                    .unwrap()
                }) {
                    Ok(datetime) => JsonValue::String(datetime.to_rfc3339()),
                    Err(_) => JsonValue::from("Invalid DateTime"),
                }
            }
            Value::Time(_neg, _days, hours, mins, secs, _usecs) => {
                // Chrono's LocalResult only implements unwrap with a panic. So we need to catch it.
                match catch_unwind(|| {
                    Utc.with_ymd_and_hms(0, 0, 0, hours as u32, mins as u32, secs as u32)
                        .unwrap()
                }) {
                    Ok(datetime) => JsonValue::String(datetime.format("%T").to_string()),
                    Err(_) => JsonValue::from("Invalid Time"),
                }
            }
            // _ => JsonValue::from("Unsupported type"),
        };
        map.insert(field, value);
    }

    Ok(map)
}
