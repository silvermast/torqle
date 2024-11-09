use async_trait::async_trait;
use tokio::net::TcpListener;
use std::sync::Arc;
use std::error::Error;
use russh_keys::{key, load_secret_key};
use russh::{client, ChannelId};
use std::time::Duration;
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

pub struct SshTunnel {
    pub opts: Arc<SshOpts>,
    pub local_port: u16,
    local_listener: TcpListener,
    ssh_session: client::Handle<SshHandler>,
}

pub struct SshHandler {
    // local_listener: &'static TcpListener,
}

#[async_trait]
impl Handler for SshHandler {
    type Error = anyhow::Error;

    // fn channel_open_confirmation(self, channel: ChannelId, max_packet_size: u32, window_size: u32, session: client::Session) -> Self::FutureUnit {
    //     println!("channel_open_confirmation: {:?}", channel);
    //     Handler::channel_open_confirmation(self, channel, max_packet_size, window_size, session)
    // }
    // fn channel_success(self, channel: ChannelId, session: Session) -> Self::FutureUnit {
    //     println!("channel_success: {:?}", channel);
    //     self.finished(session)
    // }
    // fn channel_close(self, channel: ChannelId, mut session: Session) -> Self::FutureUnit {
    //     println!("channel_close: {:?}", channel);
    //     self.local_listener;
    //     self.finished(session)
    // }
    // fn channel_eof(self, channel: ChannelId, session: Session) -> Self::FutureUnit {
    //     println!("channel_eof: {:?}", channel);
    //     self.finished(session)
    // }
    // fn data(self, channel: ChannelId, data: &[u8], session: client::Session) -> Self::FutureUnit {
    //     println!("data on channel {:?}: {:?}", channel, std::str::from_utf8(data));
    //     self.finished(session)
    // }
}

impl SshTunnel {

    // fn local_port(self) -> u16 {
    //     self.local_port
    // }
    
    // fn set_session(mut self, session: client::Handle<SshHandler>) -> Self {
    //     self.ssh_session = Some(session);
    //     self
    // }

    /**
     * Opens an SSH tunnel and returns updated driver_opts for use with connectors
     * 
     * idea: combine SshTunnel and SshHandler, and implement local tunnel handling within
     * the struct.
     *
     * - Creates a local socket listener with arbitrary port
     * - Opens a connection to the jump host
     * - Authenticates either via password or keyfile
     * - Opens a channel to the adapter target host
     * - Binds activity from the local socket to the jump host channel
     * - Returns the updated adapter_opts and the tunnel
     */
    pub async fn create(ssh_opts: SshOpts, target_host: String, target_port: u32) -> Result<Self, Box<dyn Error>> {
        println!("DEBUG: Creating SSH tunnel to {}:{} via {}:{}", target_host, target_port, ssh_opts.host, ssh_opts.port);

        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(3600)),
            ..<_>::default()
        };

        let mut ssh_session = client::connect(
            Arc::new(config),
            (ssh_opts.host.as_str(), ssh_opts.port),
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
                ssh_session
                    .authenticate_publickey(ssh_user, Arc::new(key_file))
                    .await.map_err(|e| {
                        eprintln!("Failed to authenticate with user and key: {}", e);
                        AppError::from("Failed to authenticate with the SSH Server")
                    })?
            }
            None => {
                println!("DEBUG: Authenticating with password {}:{}", ssh_user, ssh_password);
                ssh_session
                    .authenticate_password(ssh_user, ssh_password)
                    .await.map_err(|e| {
                        eprintln!("Failed to authenticate with user and password: {}", e);
                        AppError::from("Failed to authenticate with the SSH Server")
                    })?
            }
        };

        if !is_authenticated {
            return Err("Failed to authenticate with the jump host".into());
        }
        println!("DEBUG: Authenticated with jump host");

        let local_listener = TcpListener::bind(("127.0.0.1", 0)).await?;
        let local_addr = local_listener.local_addr()?;
        let local_port = local_addr.port();
        println!("DEBUG: Local listener bound to port {}", local_port);

        let mut tunnel = SshTunnel { 
            opts: Arc::new(ssh_opts),
            local_listener,
            local_port,
            ssh_session,
        };

        tunnel.open_tunnel(target_host, target_port).await?;

        Ok(tunnel)
    }

    pub async fn open_tunnel(&mut self, target_host: String, target_port: u32) -> Result<(), Box<dyn Error>> {
        let mut ssh_channel = self.ssh_session
            .channel_open_direct_tcpip(&target_host, target_port, "127.0.0.1", self.local_port as u32)
            .await?;
        println!("DEBUG: Channel open to target host");

        // ssh_channel.tcpip_forward(true, "127.0.0.1", self.local_port as u32).await?;
        // println!("DEBUG: Channel forwarding to local port {}", self.local_port);

        match self.local_listener.accept().await {
            Ok((local_stream, _)) => {
                tokio::spawn(async move {
                    match local_stream.readable().await {
                        Ok(_) => loop {
                            // let buf = AsyncReadExt::read(&mut local_stream, &mut [0; 1024]);
                            let mut buf: Vec<u8> = vec![0, 255];
                            let n = local_stream.try_read(&mut buf).unwrap_or_else(|e| {
                                eprintln!("Failed to read from local stream: {}", e);
                                0
                            });

                            println!("DEBUG: Read {} bytes from local stream", n);

                            if let Err(e) = &ssh_channel.data(&buf[..n]).await {
                                eprintln!("Failed to write to SSH channel: {}", e);
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to wait for local_stream.readable(): {}", e);
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }

        Ok(())
    }

    // async fn spawn_channel_streamers<'tunnel>(&mut self) -> Result<Self, Box<dyn Error>> {
    //     let spawn_join_handle = tokio::task::spawn(async move {
    //         let mut buf_jump_host_channel = vec![0; 2048];
    //         let mut buf_forward_stream_r = vec![0; 2048];

    //         let (mut local_socket, _) = self.local_listener.accept().await?;

    //         // let ssh_reader = self.ssh_channel.make_reader();
    //         // let ssh_writer = self.ssh_channel.make_writer();

    //         loop {
    //             // let ready = forwarder_stream_r.ready().await?;
    //             match local_socket.ready(Interest::READABLE | Interest::WRITABLE).await {
    //                 Ok(ready) => {
    //                     if ready.is_readable() {
    //                         let mut data = vec![0; 1024];
    //                         // Try to read data, this may still fail with `WouldBlock`
    //                         // if the readiness event is a false positive.
    //                         match local_socket.try_read(&mut data) {
    //                             Ok(0) => self.ssh_channel.eof().await?,
    //                             Ok(n) => self.ssh_channel.data(data[..n].into()).await?,
    //                             Err(ref e) if e.kind() == futures::io::ErrorKind::WouldBlock => {
    //                                 continue;
    //                             }
    //                             Err(e) => {
    //                                 return Err(e.into());
    //                             }
    //                         }
    //                     }
            
    //                     if ready.is_writable() {
    //                         // Try to write data, this may still fail with `WouldBlock`
    //                         // if the readiness event is a false positive.
    //                         // self.ssh_session.fuse().poll_ready().await?;
    //                         self.ssh_channel.tcpip_forward(want_reply, address, port)

    //                         match local_socket.try_write(b"hello world") {
    //                             Ok(n) => {
    //                                 println!("write {} bytes", n);
    //                             }
    //                             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    //                                 continue
    //                             }
    //                             Err(e) => {
    //                                 return Err(e.into());
    //                             }
    //                         }
    //                     }
    //                 },
    //                 Err(_) => continue,
    //             }
    //         }
    //     });

    //     Ok(*self)
    // }
}