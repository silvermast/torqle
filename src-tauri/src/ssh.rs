use ssh_jumper::model::{HostSocketParams, SshForwarderEnd};
use ssh_jumper::{
    model::{HostAddress, JumpHostAuthParams, SshTunnelParams},
    SshJumper,
};
use std::borrow::Cow;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::Path;
use tokio::sync::oneshot::Receiver;

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
    addr: SocketAddr,
    end: SshForwarderEnd,
}

/**
 * Opens an SSH tunnel and returns updated driver_opts for use with connectors
 */
pub async fn tunnel(
    adapter_opts: AdapterOpts,
    ssh_opts: SshOpts,
) -> Result<(AdapterOpts, SshTunnel), AppError> {
    
    let jump_host = resolve_host(ssh_opts.host.as_str());
    
    let target_socket = HostSocketParams {
        port: adapter_opts.port,
        address: resolve_host(adapter_opts.host.as_str()),
    };

    println!("Creating JumpHostAuthParams");
    let jump_host_auth_params: JumpHostAuthParams = match ssh_opts.keyfile {
        Some(ref ssh_key_str) => {
            println!("ssh keyfile auth: {:?}", ssh_opts);
            JumpHostAuthParams::key_pair(
                Cow::Borrowed(ssh_opts.user.as_str()),
                Cow::Borrowed(Path::new(ssh_key_str)),
                None,
            )
        }
        None => {
            println!("ssh password auth: {:?}", ssh_opts);
            JumpHostAuthParams::password(
                Cow::Borrowed(ssh_opts.user.as_str()),
                Cow::Borrowed(ssh_opts.password.as_str()),
            )
        }
    };

    // Optional: OS will allocate a port if this is left out
    let ssh_params = SshTunnelParams::new(jump_host, jump_host_auth_params, target_socket)
        .with_jump_host_port(ssh_opts.port);

    println!("SshJumper::open_tunnel");
    let jumper = SshJumper::open_tunnel(&ssh_params)
        .await
        .map_err(AppError::from)?;

    println!("Waiting for tunnel to open");
    let addr = jumper.0;
    let end = jumper.1.await.map_err(AppError::from)?;
    println!("Tunnel opened: {:?}", addr);

    let mut return_opts = adapter_opts.clone();
    return_opts.host = addr.ip().to_string();
    return_opts.port = addr.port();

    println!("Tunnel open!");
    Ok((return_opts, SshTunnel{ addr: addr, end: end }))
}
