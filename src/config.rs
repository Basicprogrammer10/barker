use std::path::Path;

use simple_config_parser;

pub struct Config {
    pub server_host: String,
    pub server_port: u16,

    pub password_salt: Vec<u8>,
    pub database_path: String,
}

impl Config {
    pub fn new<T: AsRef<Path>>(path: T) -> Option<Self> {
        let cfg = simple_config_parser::Config::new().file(path).unwrap();

        Some(Self {
            server_host: cfg.get_str("host").ok()?,
            server_port: cfg.get("port").ok()?,
            password_salt: cfg.get_str("password_salt").ok()?.as_bytes().to_vec(),
            database_path: cfg.get_str("database_path").ok()?,
        })
    }
}
