#[cfg(test)]
mod sqlite_tests {
    macro_rules! tokio_await {
        ($x:expr) => {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on($x)
        };
    }

    use crate::adapters::{self, Adapter, AdapterOpts};

    #[test]
    fn test_example() {
        assert!(true);
    }

    #[test]
    fn test_adapter() {
        let opts: AdapterOpts = AdapterOpts {
            driver: adapters::DriverType::SQLite,
            host: "".to_string(),
            port: 0,
            user: "".to_string(),
            password: "".to_string(),
            filepath: "../data/sqlite-testdb.db".to_string()
        };
        let mut adapter = tokio_await!(adapters::connect_adapter(opts, None)).unwrap();
        let result = tokio_await!(adapter.query("SELECT * FROM albums LIMIT 10".to_string(), None)).unwrap();
        tokio_await!(adapter.disconnect());

        assert_eq!("10".to_string(), result.num_rows);
    }
}