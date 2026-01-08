use anyhow::{Context, Result};
use axum::{Router, http::StatusCode, routing::get};
use sqlx::Database;
use sqlx::{PgPool, postgres::PgConnectOptions};
use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

/// Health check handler
async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Hello world handler
async fn hello_world() -> &'static str {
    "Hello World!"
}

struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    /// Read environment variables from a .env file
    fn from_env() -> Result<Self> {
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

        Ok(DatabaseConfig {
            host: "localhost".into(),
            port: db_port.parse::<u16>().unwrap(),
            username: db_username,
            password: db_password,
            database: db_name,
        })
    }
}

/// Create a struct for Postgres from DatabaseConfig
impl From<DatabaseConfig> for PgConnectOptions {
    fn from(cfg: DatabaseConfig) -> Self {
        Self::new()
            .host(&cfg.host)
            .port(cfg.port)
            .username(&cfg.username)
            .password(&cfg.password)
            .database(&cfg.database)
    }
}

/// Generate a database connection pool
fn connect_database_with(cfg: DatabaseConfig) -> PgPool {
    PgPool::connect_lazy_with(cfg.into())
}

#[tokio::main]
async fn main() {
    let database_config = DatabaseConfig::from_env().unwrap();
    let conn_pool = connect_database_with(database_config);

    // Router config
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/health", get(health_check))
        .with_state(conn_pool);

    // Listen for requests on localhost:8080
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    // Start up a listener bound to the specified address
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);

    // Start up server
    axum::serve(listener, app).await.unwrap();
}

#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;

    assert_eq!(status_code, StatusCode::OK);
}
