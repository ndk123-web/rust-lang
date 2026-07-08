use std::env;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: String,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not found"),
            port: env::var("PORT").expect("PORT NOT FOUND"),
        }
    }
}
