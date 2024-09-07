use std::collections::HashMap;

use sqlx::sqlite::{SqlitePool};
use sqlx::{Column, Row, TypeInfo};
pub use serde_json::Value as JsonValue;

use super::{Adapter, AdapterOpts, QueryResult};
use crate::AppError;

pub async fn connect(opts: AdapterOpts) -> Result<SQLiteAdapter, AppError>
where
    SQLiteAdapter: Sized,
{
    let pool = SqlitePool::connect(&opts.filepath.as_str()).await.map_err(AppError::from)?;

    Ok(SQLiteAdapter { pool: pool })
}

#[derive(Clone)]
pub struct SQLiteAdapter {
    pool: SqlitePool,
}
impl Adapter for SQLiteAdapter {
    async fn disconnect(&mut self) -> Result<bool, AppError> {
        Ok(true)
    }

    async fn query(
        &self,
        query: String,
        database: Option<String>,
    ) -> Result<QueryResult, AppError> {
        let start_time = std::time::SystemTime::now();

        let query_results = sqlx::raw_sql(&query.as_str())
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::from)?;

        let mut results: Vec<HashMap<String, JsonValue>> = Vec::with_capacity(query_results.len());

        for row in query_results {
            let mut map = HashMap::new();
            for column in row.columns() {
                let i = column.ordinal();

                let value = match column.type_info().name() {
                    "NULL" => JsonValue::Null,
                    "INTEGER" => {
                        let v: i32 = row.get(&i);
                        JsonValue::from(v)
                    },
                    "TEXT" => {
                        let v: String = row.get(&i);
                        JsonValue::from(v)
                    },
                    "REAL" => {
                        let v: f64 = row.get(&i);
                        JsonValue::from(v)
                    },
                    _ => {
                        println!("Unable to decode '{}'", column.type_info().name());
                        JsonValue::Null
                    },
                };

                map.insert(column.name().to_string(), value);
            }
            results.push(map);
        }

        Ok(QueryResult::make(start_time, Vec::from_iter(results)))
    }
}
