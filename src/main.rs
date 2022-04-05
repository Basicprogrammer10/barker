use std::sync::Arc;

use afire::prelude::*;
use parking_lot::Mutex;
use rusqlite::Connection;

mod common;
mod database;
mod routes;
mod session;

pub struct App {
    database: Mutex<Connection>,
    sessions: Mutex<Vec<session::Session>>,
    salt: Vec<u8>,
}

fn main() {
    // Create and init database connection
    let mut conn = Connection::open("data/data.db").unwrap();
    database::init(&mut conn);

    // Make app struct
    let app = Arc::new(App {
        database: Mutex::new(conn),
        sessions: Mutex::new(Vec::new()),
        salt: "Pepper".as_bytes().to_vec(),
    });

    // Make web server
    let mut server = Server::new("localhost", 8080);
    routes::attatch(&mut server, app);

    server.start().unwrap();
}
