use anyhow::{Error, Result};
use async_trait::async_trait;
use tokio::net::TcpListener;
use tokio::select;
use std::{net::Ipv4Addr, sync::Arc};
use russh::{client, ChannelId, Disconnect};
use russh_keys::{key, load_secret_key};
use std::time::Duration;
use std::net::SocketAddr;
use client::Handler;
use crate::AppError;

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct SshOpts {
    pub host: String,
    pub keyfile: Option<String>,
    pub password: String,
    pub port: u16,
    pub user: String,
}

pub struct SshHandler {
}
#[async_trait]
impl Handler for SshHandler {
    type Error = Error; 
    async fn check_server_key(&mut self, _: &key::PublicKey) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

async fn connect(ssh_opts: SshOpts) -> Result<client::Handle<SshHandler>, Error> {
    
    let config = client::Config {
        // custom configs go here. possibly add in from ssh_opts
        ..<_>::default()
    };

    let mut ssh_client = client::connect(
        Arc::new(config),
        format!("{}:{}", ssh_opts.host, ssh_opts.port),
        SshHandler {},
    )
    .await?;
    println!("DEBUG: Connected to jump host {}:{}", ssh_opts.host, ssh_opts.port);

    // Create a session to the jump host
    let ssh_user = ssh_opts.user.clone();
    let ssh_password = ssh_opts.password.clone();
    let password_option = if ssh_password.len() > 0 {
        Some(ssh_password.as_str())
    } else {
        None
    };

    let is_authenticated: bool = match ssh_opts.keyfile.clone() {
        Some(ssh_key_file) => {
            println!("DEBUG: Authenticating with keyfile");
            let key_file = load_secret_key(ssh_key_file, password_option)?;
            ssh_client
                .authenticate_publickey(ssh_user, Arc::new(key_file))
                .await.map_err(|e| {
                    eprintln!("Failed to authenticate with user and key: {}", e);
                    AppError::from("Failed to authenticate with the SSH Server")
                })?
        }
        None => {
            println!("DEBUG: Authenticating with password {}:{}", ssh_user, ssh_password);
            ssh_client
                .authenticate_password(ssh_user, ssh_password)
                .await.map_err(|e| {
                    eprintln!("Failed to authenticate with user and password: {}", e);
                    AppError::from("Failed to authenticate with the SSH Server")
                })?
        }
    };

    if !is_authenticated {
        return Err(Error::msg("Failed to authenticate with the jump host"));
    }
    
    Ok(ssh_client)
}

pub async fn jump(ssh_opts: SshOpts, target_host: String, target_port: u32) -> Result<SocketAddr, Error> {
    let ssh_client = connect(ssh_opts).await?;

    let local_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await?;
    let local_addr = local_listener.local_addr()?;
    let local_port = local_addr.port();
    
    println!("DEBUG: Local listener bound to port {}", local_port);

    let (tx, rx) = tokio::sync::watch::channel::<u8>(1);

    tokio::spawn(async move {
        loop {
            let mut rx_clone = rx.clone();
            
            if let Ok((mut local_stream, _)) = local_listener.accept().await {
                let ssh_channel = ssh_client.channel_open_direct_tcpip(
                        target_host.clone(), 
                        target_port.into(), 
                        local_addr.ip().to_string(), 
                        local_port as u32, 
                    )
                    .await?;
                println!("DEBUG: Channel open to target host");

                let mut remote_stream = ssh_channel.into_stream();

                tokio::spawn(async move {
                    select! {
                        result = tokio::io::copy_bidirectional_with_sizes(&mut local_stream, &mut remote_stream, 255, 8 * 1024) => {
                            if let Err(e) = result {
                                eprintln!("Error during bidirectional copy: {}", e);
                            }
                        }
                        _ = rx_clone.changed() => {
                            println!("Received close signal");
                        }
                    }
                });
            }
            if rx.has_changed()? {
                disconnect(&tx, &ssh_client);
            }
        }
        Ok::<(), Error>(())
    });

    Ok(local_addr.clone())
}

async fn disconnect(
    tx: &tokio::sync::watch::Sender<u8>, 
    ssh_client: &client::Handle<SshHandler>,
) -> Result<()> {
    tx.send(0)?;
    ssh_client.disconnect(Disconnect::ByApplication, "Disconnected by User", "none").await?;
    Ok(())
}
