use anyhow::{Context, Result};
use std::env;

pub struct AppConfig {
    pub database: DatabaseConfig,
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl AppConfig {
    /// Read environment variables from a .env file
    pub fn new() -> Result<Self> {
        if let Err(e) = dotenvy::dotenv()
            && !e.not_found()
        {
            return Err(e).context("Faild to load .env file");
        }

        let db_port = env::var("POSTGRES_PORT")
            .expect("DB port not set")
            .to_string();
        let db_username = env::var("POSTGRES_USER").expect("DB user name not set");
        let db_password = env::var("POSTGRES_PASSWORD").expect("DB password not set");
        let db_name = env::var("POSTGRES_DB").expect("DB name not set");

        let database = DatabaseConfig {
            host: "localhost".into(),
            port: db_port.parse::<u16>().unwrap(),
            username: db_username,
            password: db_password,
            database: db_name,
        };

        Ok(Self { database })
    }
}

#[cfg(test)]
mod tests {}
