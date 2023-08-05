use std::path::Path;
use ssh_jumper::model::{HostSocketParams, SshForwarderEnd};
use tokio::sync::oneshot::Receiver;
use std::borrow::Cow;
use ssh_jumper::{
    model::{HostAddress, JumpHostAuthParams, SshTunnelParams},
    SshJumper
};

use crate::adapters::{AdapterOpts, SshOpts, QueryError};

/**
 * Opens an SSH tunnel and returns updated driver_opts for use with connectors
 */
pub async fn tunnel(adapter_opts: AdapterOpts, ssh_opts: SshOpts) -> Result<(AdapterOpts, Receiver<SshForwarderEnd>), QueryError> {
    let jump_host = HostAddress::HostName(Cow::Borrowed(ssh_opts.host.as_str()));
    let target_socket = HostSocketParams {
        port: adapter_opts.port,
        address: HostAddress::HostName(Cow::Borrowed(adapter_opts.host.as_str())),
    };

    println!("Creating JumpHostAuthParams");
    let jump_host_auth_params: JumpHostAuthParams = match ssh_opts.keyfile {
        Some(ref ssh_key_str) => {
            println!("ssh keyfile auth: {:?}", ssh_opts);
            JumpHostAuthParams::key_pair(
                Cow::Borrowed(ssh_opts.user.as_str()), 
                Cow::Borrowed(Path::new(ssh_key_str)),
                None
            )
        },
        None => {
            println!("ssh password auth: {:?}", ssh_opts);
            JumpHostAuthParams::password(
                Cow::Borrowed(ssh_opts.user.as_str()), 
                Cow::Borrowed(ssh_opts.password.as_str())
            )
        }
    };

    // Optional: OS will allocate a port if this is left out
    let ssh_params = SshTunnelParams::new(jump_host, jump_host_auth_params, target_socket).with_jump_host_port(ssh_opts.port);

    println!("SshJumper::open_tunnel");
    let jumper = SshJumper::open_tunnel(&ssh_params).await
        .map_err(|why| QueryError::from(why))?;

    let mut return_opts = adapter_opts.clone();
    return_opts.host = jumper.0.ip().to_string();
    return_opts.port = jumper.0.port();

    println!("Tunnel open!");
    Ok((return_opts, jumper.1))
}