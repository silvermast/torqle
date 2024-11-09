use ssh_jumper::model::HostSocketParams;
use ssh_jumper::{
    model::{HostAddress, JumpHostAuthParams, SshTunnelParams},
    SshJumper,
};
use std::borrow::Cow;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::Path;
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
    // end: SshForwarderEnd,
}
impl SshTunnel {
    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }
    // pub fn end(&self) -> &SshForwarderEnd {
    //     &self.end
    // }
}

/**
 * Opens an SSH tunnel and returns updated driver_opts for use with connectors
 */
pub async fn tunnel<'tunnel>(
    adapter_opts: &'tunnel AdapterOpts,
    ssh_opts: &'tunnel SshOpts,
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

    let jumper = SshJumper::open_tunnel(&ssh_params).await.map_err(AppError::from)?;

    let addr = jumper.0.clone();
    // let end = jumper.1.await.map_err(AppError::from)?;

    let mut return_opts = adapter_opts.clone();
    return_opts.host = addr.ip().to_string();
    return_opts.port = addr.port();

    let tunnel = SshTunnel{ addr: jumper.0 };

    println!("Tunnel open!");
    Ok((return_opts, tunnel))
}


#[cfg(test)]
mod ssh_tests {
    macro_rules! tokio_await {
        ($x:expr) => {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on($x)
        };
    }

    

    use super::tunnel;
    use crate::adapters::{self, AdapterOpts, SshOpts};

    #[test]
    fn test_example() {
        assert!(true);
    }

    #[test]
    fn test_ssh_jumper() {
        use std::borrow::Cow;
        use ssh_jumper::{
            model::{HostAddress, HostSocketParams, JumpHostAuthParams, SshTunnelParams},
            SshJumper
        };

        let (local_socket_addr, _ssh_forwarder_end_rx) = {
            let jump_host = HostAddress::HostName(Cow::Borrowed("localhost"));
            let jump_host_auth_params = JumpHostAuthParams::password(
                Cow::Borrowed("torqle"),
                Cow::Borrowed("littlebuddy"),
            );
            let target_socket = HostSocketParams {
                address: HostAddress::HostName(Cow::Borrowed("mysql")),
                port: 3306,
            };
            let ssh_params =
                SshTunnelParams::new(jump_host, jump_host_auth_params, target_socket)
                    // Optional: OS will allocate a port if this is left out
                    .with_jump_host_port(10022);

            tokio_await!(SshJumper::open_tunnel(&ssh_params)).unwrap()
        };

        assert!(local_socket_addr.port() > 0);
    }

    #[test]
    fn test_tunnel() {
        let adapter_opts = AdapterOpts {
            driver: adapters::DriverType::MySQL,
            host: "mysql".to_string(),
            port: 3306,
            user: "root".to_string(),
            password: "mypassword".to_string(),
            filepath: "".to_string(),
        };
        let ssh_opts: SshOpts = SshOpts {
            host: "127.0.0.1".to_string(),
            port: 10022,
            user: "torqle".to_string(),
            password: "littlebuddy".to_string(),
            keyfile: None,
        };

        // let panic_result = panic::catch_unwind( || {
        //     let (new_opts, _ssh_tunnel) = tokio_await!(tunnel(adapter_opts, ssh_opts)).unwrap();
        //     assert_eq!("127.0.0.1".to_string(), new_opts.host);
        // });
        // println!("Panic result: {:?}", panic_result);
        let (new_opts, _ssh_tunnel) = tokio_await!(tunnel(&adapter_opts, &ssh_opts)).unwrap();
        assert_eq!("127.0.0.1".to_string(), new_opts.host);

        // let (new_opts, _ssh_tunnel) = tokio_await!(tunnel(adapter_opts, ssh_opts)).unwrap();
    }
}