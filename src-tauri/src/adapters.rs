use std::collections::HashMap;
use serde::Serialize;
use sqlx::{AnyPool, Row, Executor, any::{AnyRow, AnyValue}, Column};
use ssh_jumper::model::SshForwarderEnd;
use tokio::sync::oneshot::Receiver;

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
    pub rows: Vec<HashMap<String, String>>,
}

#[derive(Serialize, Debug)]
pub struct QueryError {
    // query: String,
    pub error: String,
}
impl QueryError {
    pub fn from<E: std::fmt::Display>(err: E) -> QueryError {
        QueryError { error: format!("{}", err) }
    }
}
#[derive(Clone)]
pub struct Adapter {
    pool: AnyPool,
    // ssh_tunnel: Option<Receiver<SshForwarderEnd>>,
}
impl Adapter {
    pub async fn connect(opts: AdapterOpts, ssh_opts: Option<SshOpts>) -> Result<Self, QueryError> where Self: Sized {
        sqlx::any::install_default_drivers();

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
            // ssh_tunnel: ssh_tunnel,
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
        let query_results = conn.fetch_all(query.as_str()).await.map_err(QueryError::from)?;

        let results = query_results.iter().map(|row: &AnyRow| {
            let columns = row.columns();
            let mut row_map: HashMap<String, String> = HashMap::new();
            for index in 0..columns.len() {
                let field: String = columns[index].name.to_string();
                let value: String = row.get(field.as_str());
                row_map.insert(field, value);
            }
            row_map
        });

        // let results = query_results.iter().map(|row: AnyRow| {
        //     row.
        // })
        // for index in 0..query_results.len() {

        // }
        // let results = sqlx::query(query.as_str())
        //     .fetch(&mut conn)
        //     .await?;
        // .map(|row: AnyRow| {
        //     let mut row_map = HashMap::new();
        //     let columns = row.columns();
        //     for index in 0..columns.len() {
        //         let field: String = columns.get(index).unwrap().;
        //         let value: String = row.get(index).into(); // row[index].into();
        //         row_map.insert(field, value);
        //     }
        //     row_map
        // })
        
        // let results = conn.query_map(query, |mut row: Row| {
        //     let mut row_map = HashMap::new();
        //     let columns = row.columns();
        //     for index in 0..columns.len() {
        //         let field: String = columns[index].name_str().into();
        //         let value: String = row.take(index).unwrap_or("".to_string());
        //         row_map.insert(field, value);
        //     }
        //     row_map
        // })?;

        Ok(QueryResult {
            elapsed_ms: start_time.elapsed().expect("Error parsing elapsed timestamp!").as_millis().to_string(),
            num_rows: results.len().to_string(),
            rows: Vec::from_iter(results),
        })
    }
}

