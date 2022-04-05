use std::sync::Arc;

use afire::prelude::*;
use parking_lot::Mutex;
use rusqlite::Connection;

mod common;
mod config;
mod database;
mod routes;
mod session;

pub struct App {
    database: Mutex<Connection>,
    sessions: Mutex<Vec<session::Session>>,
    config: config::Config,
}

fn main() {
    // Load config
    let cfg = config::Config::new("data/config.cfg").unwrap();

    // Create and init database connection
    let mut conn = Connection::open(&cfg.database_path).unwrap();
    database::init(&mut conn);

    // Make app struct
    let app = Arc::new(App {
        database: Mutex::new(conn),
        sessions: Mutex::new(Vec::new()),
        config: cfg,
    });

    // Make web server
    let mut server = Server::new(&app.config.server_host, app.config.server_port);
    routes::attatch(&mut server, app);

    server.start().unwrap();
}
