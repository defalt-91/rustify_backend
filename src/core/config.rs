use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;

use axum::http::{HeaderName, HeaderValue, Method};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, X_CONTENT_TYPE_OPTIONS};
use diesel_async::AsyncPgConnection;
use jsonwebtoken::Algorithm;

pub fn base_dir() -> PathBuf { env::current_dir().unwrap() }

pub fn log_level() -> String { env::var("LOG_LEVEL").unwrap() }

pub fn bind() -> SocketAddrV4 {
    let host: Ipv4Addr = env::var("HOST").unwrap().parse::<Ipv4Addr>().expect("HOST is required");
    let port: u16 = env::var("PORT").unwrap().parse::<u16>().expect("PORT is required");
    SocketAddrV4::new(host, port)
}

pub fn secret() -> String {
    env::var("SECRET").expect("secret is missing!")
}

pub fn jwt_key() -> String {
    env::var("JWT_KEY").unwrap()
}

pub  fn jwt_algorithm() -> Algorithm {
    env::var("JWT_ALGORITHM").unwrap().parse().unwrap()
}


pub  fn jwt_exp_secs() -> u64 {
    env::var("JWT_EXP_SECS").unwrap().parse().unwrap()
}

pub fn pg_user() -> String {
    env::var("DATABASE_USER").unwrap()
}

pub fn pg_pass() -> String {
    env::var("DATABASE_PASS").unwrap()
}

pub fn pg_path() -> String {
    env::var("DATABASE_PATH").unwrap()
}
pub fn pg_db() -> String {
    env::var("PG_DB").unwrap()
}
pub fn pg_url() -> String {
    let user = pg_user();
    let pass = pg_pass();
    let pg_path = pg_path();
    let pg_db = pg_db();
    format!("{user}:{pass}//{pg_path}/{pg_db}")
}

pub fn allow_origin() -> Vec<HeaderValue> {
    env::var("ALLOWED_ORIGINS").unwrap().split(",").map(|v| v.parse::<HeaderValue>().unwrap()).collect()
}

pub fn allow_methods() -> [Method; 6] {
    [
        Method::GET,
        Method::POST,
        Method::PATCH,
        Method::DELETE,
        Method::HEAD,
        Method::OPTIONS,
    ]
}

pub fn allow_headers() -> [HeaderName; 5] {
    [AUTHORIZATION, ACCEPT, CONTENT_TYPE, X_CONTENT_TYPE_OPTIONS, CONTENT_LENGTH]
}
