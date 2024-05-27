use ssh_jumper::model::SshForwarderEnd;
use std::borrow::Borrow;
use std::collections::HashMap;
use tokio::sync::oneshot::Receiver;

use serde::Serialize;
pub use serde_json::Map as JsonMap;
pub use serde_json::Value as JsonValue;

use crate::ssh;
use crate::AppError;

mod mysql;
mod sqlite;

use mysql::MySQLAdapter;
use sqlite::SQLiteAdapter;

/**
 * @see https://github.com/jasondeewright/sqlx
 */

#[derive(Clone, Debug, serde::Deserialize, Copy, Default)]
pub enum DriverType {
    MySQL,
    PostgreSQL,
    SQLite,
    MongoDB,
    #[default]
    None,
}

/**
 * @todo add polymorphism to AdapterOpts
 */
#[derive(Clone, Debug, serde::Deserialize, Default)]
pub struct AdapterOpts {
    pub driver: DriverType,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub filepath: String,
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
    pub fields: Vec<String>,
    pub rows: Vec<HashMap<String, JsonValue>>,
}

#[derive(Clone)]
pub enum AdapterEnum {
    SQLite(SQLiteAdapter),
    MySQL(MySQLAdapter),
}

// #[derive(Default, Clone)]
pub trait Adapter {
    // async fn connect(&self, opts: AdapterOpts) -> Result<Self, AppError> where Self: Sized;
    async fn query(&self, query: String, database: Option<String>)
        -> Result<QueryResult, AppError>;
    async fn disconnect(&mut self) -> Result<bool, AppError>;
}

impl Adapter for AdapterEnum {
    async fn query(
        &self,
        query: String,
        database: Option<String>,
    ) -> Result<QueryResult, AppError> {
        match self {
            AdapterEnum::MySQL(adapter) => adapter.query(query, database).await,
            AdapterEnum::SQLite(adapter) => adapter.query(query, database).await,
            _ => Err(AppError::from("Driver not found!")),
        }
    }

    async fn disconnect(&mut self) -> Result<bool, AppError> {
        match self {
            AdapterEnum::MySQL(adapter) => adapter.disconnect().await,
            AdapterEnum::SQLite(adapter) => adapter.disconnect().await,
            _ => Ok(true),
        }
    }
}

pub async fn connect_adapter(
    opts: AdapterOpts,
    ssh_opts: Option<SshOpts>,
) -> Result<AdapterEnum, AppError> {
    let (driver_opts, ssh_tunnel): (AdapterOpts, Option<Receiver<SshForwarderEnd>>) = match ssh_opts
    {
        Some(ssh_opts_actual) => {
            let (new_opts, ssh_tunnel) = ssh::tunnel(opts, ssh_opts_actual).await?;
            (new_opts, Some(ssh_tunnel))
        }
        None => (opts, None),
    };

    match driver_opts.driver {
        DriverType::MySQL => Ok(AdapterEnum::MySQL(mysql::connect(driver_opts).await?)),
        DriverType::SQLite => Ok(AdapterEnum::SQLite(sqlite::connect(driver_opts).await?)),
        _ => Err(AppError::from("Unable to connect: unknown driver!")),
    }
}
