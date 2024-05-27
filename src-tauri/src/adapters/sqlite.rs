use async_sqlite::{JournalMode, Pool, PoolBuilder};
pub use serde_json::Map as JsonMap;
pub use serde_json::Value as JsonValue;

use super::{Adapter, AdapterOpts, QueryResult};
use crate::AppError;

pub async fn connect(opts: AdapterOpts) -> Result<SQLiteAdapter, AppError>
where
    SQLiteAdapter: Sized,
{
    /** @todo fix journal_mode to pass in as param */
    let journal_mode = match opts.filepath.as_str() {
        ":memory:" => JournalMode::Memory,
        _ => JournalMode::Delete,
    };

    let pool = PoolBuilder::new()
        .path(opts.filepath)
        .journal_mode(journal_mode)
        .open()
        .await
        .map_err(AppError::from)?;

    Ok(SQLiteAdapter { pool: pool })
}

#[derive(Clone)]
pub struct SQLiteAdapter {
    pool: Pool,
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
        let results = vec![];
        let fields = vec![];
        Ok(QueryResult {
            elapsed_ms: start_time
                .elapsed()
                .expect("Error parsing elapsed timestamp!")
                .as_millis()
                .to_string(),
            num_rows: results.len().to_string(),
            rows: Vec::from_iter(results),
            fields: fields,
        })
    }
}
