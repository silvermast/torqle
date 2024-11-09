use std::borrow::Cow;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::sync::Arc;
use std::thread;
use ssh2::{Channel, Session};

use crate::adapters::{AdapterOpts, SshOpts};
use crate::AppError;

fn resolve_host(host: &str) -> HostAddress {
    if let Ok(ip) = host.parse::<Ipv4Addr>() {
        HostAddress::IpAddr(ip.into())
    } else if let Ok(ip) = host.parse::<Ipv6Addr>() {
        HostAddress::IpAddr(ip.into())
    } else {
        HostAddress::HostName(Cow::Borrowed(host))
    }
}

#[derive(Debug)]
pub struct SshTunnel {
    host: String,
    port: u16,
    tunnel: Channel,
    session: Session,
}

fn bind_channel_to_local_stream(
    channel: &Channel,
    target_host: &str,
    target_port: u16,
    local_listener: TcpListener,
) -> Result<(), Box<dyn std::error::Error>> {
    
    
    Ok(())
}

/**
 * Opens an SSH tunnel and returns updated driver_opts for use with connectors
 */
pub async fn tunnel(
    adapter_opts: AdapterOpts,
    ssh_opts: SshOpts,
) -> Result<(AdapterOpts, SshTunnel), AppError> {
    // Connect to the local SSH server
    let jump_host = format!("{}:{}", ssh_opts.host, ssh_opts.port);
    let tcp = TcpStream::connect(jump_host).map_err(AppError::from)?;
    let mut sess = Session::new().map_err(AppError::from)?;
    sess.set_tcp_stream(tcp);
    sess.handshake().map_err(AppError::from)?;

    // Try to authenticate with the first identity in the agent.
    if let Some(ssh_key_file) = ssh_opts.keyfile {
        sess.userauth_pubkey_file(&ssh_opts.user, None, &Path::new(&ssh_key_file), None).map_err(AppError::from)?;
    } else if (ssh_opts.password.len() > 0) {
        sess.userauth_password(&ssh_opts.user, &ssh_opts.password).map_err(AppError::from)?;
    } else {
        sess.userauth_agent(&ssh_opts.user).unwrap();
    }

    if !sess.authenticated() {
        return Err(AppError::from("Failed to authenticate with SSH server"));
    }

    let channel = sess.channel_direct_tcpip(&adapter_opts.host, adapter_opts.port, None).map_err(AppError::from)?;

    let local_stream = TcpStream::connect("")?;
    local_stream.peer_addr()?.

    // Accept incoming connections on the local listener
    for stream in local_listener.incoming() {
        match stream {
            Ok(mut local_stream) => {
                // Create an SSH channel to the target host
                // let mut channel = session.channel_direct_tcpip(target_host, target_port, None)?;
                
                // Spawn threads to forward data between the local stream and the SSH channel
                thread::spawn(move || {
                    let mut buffer = [0u8; 1024];
                    while let Ok(n) = local_stream.read(&mut buffer) {
                        if n == 0 {
                            break;
                        }
                        channel.write_all(buffer[..n].as_ref()).unwrap();
                        channel.write_all(&buffer[..n]).unwrap();
                    }
                });
                
                let mut local_stream_clone = local_stream.try_clone().unwrap();
                thread::spawn(move || {
                    let mut buffer = [0u8; 1024];
                    while let Ok(n) = channel.read(&mut buffer) {
                        if n == 0 {
                            break;
                        }
                        local_stream_clone.write_all(&buffer[..n]).unwrap();
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept incoming connection: {}", e);
            }
        }
    }


    let mut return_opts = adapter_opts.clone();
    return_opts.host = "127.0.0.1".to_string();
    return_opts.port = channel;

    println!("Tunnel open!");
    Ok((return_opts, SshTunnel { host: ssh_opts.host, port: ssh_opts.port, tunnel: channel, session: sess }))
}


#[cfg(test)]
mod ssh_tests {
    macro_rules! tokio_await {
        ($x:expr) => {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on($x)
        };
    }

    use super::tunnel;
    use crate::adapters::{self, Adapter, AdapterOpts, SshOpts};

    #[test]
    fn test_example() {
        assert!(true);
    }

    #[test]
    fn test_tunnel() {
        let adapter_opts = AdapterOpts {
            driver: adapters::DriverType::SQLite,
            host: "".to_string(),
            port: 0,
            user: "".to_string(),
            password: "".to_string(),
            filepath: "../data/sqlite-testdb.db".to_string(),
        };
        let ssh_opts: SshOpts = SshOpts {
            host: "127.0.0.1".to_string(),
            port: 10022,
            user: "root".to_string(),
            password: "littlebuddy".to_string(),
            keyfile: None,
        };

        let (new_opts, ssh_tunnel) = tokio_await!(tunnel(adapter_opts, ssh_opts)).unwrap();
        assert_eq!("127.0.0.1".to_string(), new_opts.host);
    }
}