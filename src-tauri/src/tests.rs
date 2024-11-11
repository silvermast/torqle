#![allow(unused_must_use)]
#[cfg(test)]
mod torqle_tests {
    use crate::adapters::{self, Adapter, AdapterOpts};
    use crate::ssh;
    use crate::ssh::SshOpts;

    #[test]
    fn test_example() {
        assert!(true);
    }

    #[tokio::test]
    async fn test_adapter() {
        let opts: AdapterOpts = AdapterOpts {
            driver: adapters::DriverType::SQLite,
            filepath: "../data/sqlite-testdb.db".to_string(),
            ..<_>::default()
        };
        let mut adapter = adapters::connect_adapter(opts, None).await.unwrap();
        let result = adapter
            .query("SELECT * FROM albums LIMIT 10".to_string(), None)
            .await
            .unwrap();
        adapter.disconnect().await;

        assert_eq!("10".to_string(), result.num_rows);
    }

    #[tokio::test]
    async fn test_ssh_tunnel_password() {
        let ssh_opts: SshOpts = SshOpts {
            host: "localhost".to_string(),
            port: 10022,
            user: "torqle".to_string(),
            password: "littlebuddy".to_string(),
            keyfile: None,
        };

        let tunnel = ssh::jump(ssh_opts, "mysql".to_string(), 3306)
            .await
            .unwrap();
        assert!(tunnel.port() > 0);
    }

    #[tokio::test]
    async fn test_adapter_through_tunnel() {
        let adapter_opts: AdapterOpts = AdapterOpts {
            driver: adapters::DriverType::MySQL,
            host: "mysql".to_string(),
            port: 3306,
            user: "root".to_string(),
            password: "mypassword".to_string(),
            ..<_>::default()
        };
        let ssh_opts: SshOpts = SshOpts {
            host: "127.0.0.1".to_string(),
            port: 10022,
            user: "torqle".to_string(),
            password: "littlebuddy".to_string(),
            keyfile: None,
        };
        let mut adapter = adapters::connect_adapter(adapter_opts, Some(ssh_opts))
            .await
            .unwrap();
        let result = adapter
            .query("SELECT NOW()".to_string(), None)
            .await
            .unwrap();
        adapter.disconnect().await;

        assert_eq!("1".to_string(), result.num_rows);
    }
}
