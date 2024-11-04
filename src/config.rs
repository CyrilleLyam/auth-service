use lazy_static::lazy_static;
use std::env;

fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => panic!("{} must be set", key),
    }
}

lazy_static! {
    pub static ref HOST: String = get_env("HOST");
    pub static ref PORT: String = get_env("PORT");
    pub static ref DATABASE_URL: String = get_env("DATABASE_URL");
    pub static ref JWT_SECRET: String = get_env("JWT_SECRET");
}
