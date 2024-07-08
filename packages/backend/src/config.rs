#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub pg_conn_str: String,
}

impl Config {
    pub fn init() -> Config {
        let host = std::env::var("SERVER.HOST").expect("SERVER.HOST must be set");
        let port = std::env::var("SERVER.PORT").expect("SERVER.PORT must be set");
        let pg_conn_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Config {
            port: port.parse::<u16>().unwrap(),
            host,
            pg_conn_str,
        }
    }
}