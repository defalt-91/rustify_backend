use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;

pub fn base_dir() -> PathBuf { env::current_dir().unwrap() }

pub fn log_level() -> String { env::var("LOG_LEVEL").unwrap() }

pub fn bind() -> SocketAddrV4 {
    let host:Ipv4Addr = env::var("HOST").unwrap().parse::<Ipv4Addr>().expect("HOST is required");
    let port :u16= env::var("PORT").unwrap().parse::<u16>().expect("PORT is required");
    SocketAddrV4::new(host,port)
}

pub fn secret() -> String {
    env::var("secret").expect("secret is missing!")
}

pub fn jwt_key() -> String {
    env::var("jwt_key").unwrap()
}

pub fn surreal_user() -> String {
    env::var("surreal_user").unwrap()
}

pub fn surreal_pass() -> String {
    env::var("surreal_pass").unwrap()
}

pub fn surreal_bind() -> Ipv4Addr {
    env::var("surreal_bind").unwrap().parse().unwrap()
}

