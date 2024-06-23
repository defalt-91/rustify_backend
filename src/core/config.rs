use std::env;

use axum::http::header::{
    ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, X_CONTENT_TYPE_OPTIONS,
};
use axum::http::{HeaderName, HeaderValue, Method};
use jsonwebtoken::Algorithm;
use tokio::sync::OnceCell;

// Define a struct to represent server configuration
#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    log_level: String,
    secret: String,
    allow_origin: Vec<HeaderValue>,
    jwt_key: String,
    jwt_algorithm: Algorithm,
    allow_methods: Vec<Method>,
    jwt_exp_minutes: i64,
    allow_headers: Vec<HeaderName>,
    hash_cost:u32,
    timeout_secs:u64,
}

#[derive(Debug)]
struct DatabaseConfig {
    db_user: String,
    db_pass: String,
    db_host: String,
    db_name: String,
    db_port: String,
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    db: DatabaseConfig,
}

impl Config {
    // Getter method for the database URL
    pub fn db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            &self.db.db_user,
            &self.db.db_pass,
            &self.db.db_host,
            &self.db.db_port,
            &self.db.db_name
        )
        // format!("postgres://{}:{}@/{}", &self.db.db_user, &self.db.db_pass, &self.db.db_name)
    }

    // Getter method for the server host
    pub fn bind(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
    pub fn hash_cost(&self) -> u32 { self.server.hash_cost }
    pub fn timeout_secs(&self) -> u64 { self.server.timeout_secs }

    // Getter method for the server port
    pub fn log_level(&self) -> &str {
        &self.server.log_level
    }
    pub fn secret(&self) -> &str {
        &self.server.secret
    }
    pub fn allow_origin(&self) -> Vec<HeaderValue> {
        self.server.allow_origin.clone()
    }
    pub fn jwt_key(&self) -> &str {
        self.server.jwt_key.as_str()
    }
    pub fn jwt_algorithm(&self) -> Algorithm {
        self.server.jwt_algorithm
    }
    pub fn allow_methods(&self) -> Vec<Method> {
        self.server.allow_methods.clone()
    }
    pub fn jwt_exp_minutes(&self) -> i64 { self.server.jwt_exp_minutes }
    pub fn allow_headers(&self) -> Vec<HeaderName> {
        self.server.allow_headers.clone()
    }
}

// Create a static OnceCell to store the application configuration
pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

// Asynchronously initialize the configuration
async fn init_config() -> Config {
    // Load environment variables from a development.env file if present
    dotenv::from_path("./development.env").ok();
    // Create a ServerConfig instance with default values or values from environment variables
    let server_config = ServerConfig {
        host: env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
        port: env::var("BACKEND_PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()
            .unwrap(),
        log_level: env::var("LOG_LEVEL").unwrap(),
        secret: env::var("SECRET").unwrap_or("secret is missing!".to_string()),
        hash_cost:env::var("HASH_COST").expect("HASH_COST must be set ").parse().unwrap(),
        timeout_secs:env::var("APP_REQUEST_TIMEOUT_SECONDS").expect("APP_REQUEST_TIMEOUT_SECONDS must be set ").parse().unwrap(),
        allow_origin: env::var("ALLOWED_ORIGINS")
            .unwrap_or("[*]".to_string())
            .split(",")
            .map(|v| v.parse::<HeaderValue>().unwrap())
            .collect(),
        jwt_key: env::var("JWT_KEY").unwrap_or("change this".to_string()),
        jwt_algorithm: env::var("JWT_ALGORITHM")
            .unwrap_or("HS256".to_string())
            .parse()
            .unwrap(),
        allow_methods: vec![
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
        ],
        jwt_exp_minutes: env::var("JWT_EXP_MINUTES")
            .unwrap_or(180.to_string())
            .parse()
            .unwrap(),
        allow_headers: vec![
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
            X_CONTENT_TYPE_OPTIONS,
            CONTENT_LENGTH,
        ],
    };

    // Create a DatabaseConfig instance with a required DATABASE_URL environment variable
    let database_config = DatabaseConfig {
        db_user: env::var("POSTGRES_USER").expect("POSTGRES_USER must be set"),
        db_pass: env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set"),
        db_host: env::var("PG_HOST").expect("PG_HOST must be set"),
        db_name: env::var("POSTGRES_DB").expect("POSTGRES_DB must be set"),
        db_port: env::var("PG_PORT").expect("PG_PORT must be set"),
    };

    // Create a Config instance by combining server and database configurations
    Config {
        server: server_config,
        db: database_config,
    }
}

// Asynchronously retrieve the application configuration, initializing it if necessary
pub async fn get_config() -> &'static Config {
    // Get the configuration from the OnceCell or initialize it if it hasn't been set yet
    CONFIG.get_or_init(init_config).await
}
