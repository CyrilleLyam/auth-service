#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub database_url: String,
}

impl Config {
    pub fn init() -> Config {
        let port: String = std::env::var("PORT").expect("PORT must be set");
        let host: String = std::env::var("HOST").expect("HOST must be set");
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Config {
            port: port.parse::<u16>().unwrap(),
            host,
            database_url,
        }
    }
}
