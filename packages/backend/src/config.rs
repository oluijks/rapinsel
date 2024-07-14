#[derive(Debug, Clone)]
pub struct Config {
    pub version: String,
    pub port: u16,
    pub host: String,
    pub pg_conn_str: String,
}

impl Config {
    pub fn init() -> Config {
        let version = std::env::var("API_VERSION").expect("API_VERSION must be set");
        let host = std::env::var("SERVER_HOST").expect("SERVER_HOST must be set");
        let port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
        let pg_conn_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Config {
            version,
            port: port.parse::<u16>().unwrap(),
            host,
            pg_conn_str,
        }
    }
}
