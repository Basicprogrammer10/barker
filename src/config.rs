use std::path::Path;

pub struct Config {
    pub server_host: String,
    pub server_port: u16,

    pub password_salt: Vec<u8>,
    pub database_path: String,
    pub max_message_len: usize,
    pub session_timeout: u64,
}

impl Config {
    pub fn new<T: AsRef<Path>>(path: T) -> Option<Self> {
        let cfg = simple_config_parser::Config::new().file(path).unwrap();

        Some(Self {
            server_host: cfg.get_str("host").ok()?,
            server_port: cfg.get("port").ok()?,
            password_salt: cfg.get_str("password_salt").ok()?.as_bytes().to_vec(),
            database_path: cfg.get_str("database_path").ok()?,
            max_message_len: cfg.get::<usize>("max_message_len").ok()?,
            session_timeout: cfg.get::<u64>("session_timeout").ok()?,
        })
    }
}
