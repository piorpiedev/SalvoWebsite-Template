use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DbConfig {
    /// Settings for the primary database. This is usually writeable, but will be read-only in
    /// some configurations.
    /// An optional follower database. Always read-only.
    pub url: String,

    pub pool_size: u32,
    pub min_idle: Option<u32>,

    /// Number of seconds to wait for unacknowledged TCP packets before treating the connection as
    /// broken. This value will determine how long crates.io stays unavailable in case of full
    /// packet loss between the application and the database: setting it too high will result in an
    /// unnecessarily long outage (before the unhealthy database logic kicks in), while setting it
    /// too low might result in healthy connections being dropped.
    pub tcp_timeout: u64,
    /// Time to wait for a connection to become available from the connection
    /// pool before returning an error.
    /// Time to wait for a connection to become available from the connection
    /// pool before returning an error.
    pub connection_timeout: u64,
    /// Time to wait for a query response before canceling the query and
    /// returning an error.
    pub statement_timeout: u64,
    /// Number of threads to use for asynchronous operations such as connection
    /// creation.
    pub helper_threads: usize,
    /// Whether to enforce that all the database connections are encrypted with TLS.
    pub enforce_tls: bool,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: "postgres://dev:CHANGE-ME@127.0.0.1:5432/example_db".to_owned(),
            pool_size: 10,
            min_idle: None,
            tcp_timeout: 10000,
            connection_timeout: 30000,
            statement_timeout: 30000,
            helper_threads: 10,
            enforce_tls: false,
        }
    }
}
