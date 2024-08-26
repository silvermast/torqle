use std::collections::HashMap;

use sqlx::sqlite::{SqlitePool, SqliteRow, SqliteValue};
use sqlx::{Column, Row, Value};
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
        print!("Running query in db {:?} {}", database, query);
        let start_time = std::time::SystemTime::now();

        let query_results = sqlx::query(&query.as_str())
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::from)?;

        let mut results: Vec<HashMap<String, JsonValue>> = Vec::new();

        for row in query_results {
            let row_map = parse_row(&row)?;
            results.push(row_map);
        }

        Ok(QueryResult::make(start_time, Vec::from_iter(results)))
    }
}


fn parse_row(row: &SqliteRow) -> Result<HashMap<String, JsonValue>, AppError> {
    let mut map = HashMap::new();
    let columns = row.columns();

    for column in columns {
        let column_name = column.name().to_string();
        // let value: String = row.get(column.ordinal()).unwrap_or("".to_string());
        let value: JsonValue = row.try_get(column.ordinal()).map_err(AppError::from)?;

        // let value_raw = row.try_get_raw(column.ordinal()).map_err(AppError::from)?;
        // let value = match value_raw {
        //     sqlx::types::Type::Null => JsonValue::Null,
        //     sqlx::types::Type::Integer => JsonValue::Number(row.get::<i64, _>(column.ordinal()).into()),
        //     sqlx::types::Type::Real => JsonValue::Number(row.get::<f64, _>(column.ordinal()).into()),
        //     sqlx::types::Type::Text => JsonValue::String(row.get::<String, _>(column.ordinal())),
        //     sqlx::types::Type::Blob => JsonValue::String(hex::encode(row.get::<Vec<u8>, _>(column.ordinal()))),
        // };

        map.insert(column_name, value);
    }

    Ok(map)
}
