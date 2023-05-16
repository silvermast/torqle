use std::collections::HashMap;
use std::error::Error;
use serde::Serialize;

pub mod mysql;

pub enum Drivers {
    Mysql,
    Postgres,
    Mongodb,
    Sqlite,
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


pub trait ClientConnection {
    fn connect(opts: HashMap<String, String>) -> Result<Self, Box<dyn Error>> where Self: Sized;
    fn test(opts: HashMap<String, String>) -> Result<bool, Box<dyn Error>> where Self: Sized;
    // fn connect() -> Result<bool, String>;
    fn disconnect(&mut self) -> bool;
    fn query(&self, query: String) -> Result<QueryResult, QueryError>;
    fn change_schema(&mut self, value: String) -> Result<bool, Box<dyn Error>>;
    fn get_opt(&self, option: String) -> Option<String>;
    fn set_opt(&mut self, option: String, value: String) -> Result<bool, Box<dyn Error>> where Self: Sized;
}

/* ChatGPT Example

use std::io::Error;
use thrussh::{client, ChannelId, ClientSession, Session};
use thrussh_keys::{key::KeyPair, load_secret_key};
use mysql::{Opts, OptsBuilder, Pool};

async fn connect_to_mysql_via_ssh_tunnel(
    ssh_host: &str,
    ssh_username: &str,
    ssh_key_path: &str,
    ssh_port: u16,
    mysql_host: &str,
    mysql_username: &str,
    mysql_password: &str,
    mysql_database: &str,
    mysql_port: u16,
) -> Result<Pool, Error> {
    // Load the private SSH key
    let private_key = load_secret_key(ssh_key_path).unwrap();

    // Connect to the SSH server
    let tcp = std::net::TcpStream::connect((ssh_host, ssh_port))?;
    let config = client::Config::default();
    let mut session = ClientSession::new(&config, tcp, ssh_host);
    session.handshake().await?;

    // Authenticate with the SSH server using the private key
    let auth = session
        .auth_publickey(ssh_username, KeyPair::from_secret_key(private_key))
        .await?;
    assert!(auth);

    // Open a TCP connection to the MySQL server via the SSH tunnel
    let mut channel = session.channel_open_direct_tcpip(mysql_host, mysql_port, None).await?;
    let _id = channel.id();

    // Connect to the MySQL server using the TCP connection
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(mysql_host))
        .user(Some(mysql_username))
        .pass(Some(mysql_password))
        .db_name(Some(mysql_database))
        .tcp_port(mysql_port)
        .tcp_connect_timeout(Some(std::time::Duration::from_secs(10)))
        .ssl_opts(None::<mysql::SslOpts>);
    let pool = Pool::new(Opts::from(opts))?;

    Ok(pool)
}

let pool = connect_to_mysql_via_ssh_tunnel(
    "ssh.example.com",
    "sshuser",
    "/path/to/ssh/key",
    22,
    "mysql.example.com",
    "mysqluser",
    "mysqlpassword",
    "mysqldb",
    3306,
).await.unwrap();
*/

// struct MongodbConnection {}
// struct SqliteConnection {}
// struct PostgresConnection {}
