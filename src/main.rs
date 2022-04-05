use std::sync::Arc;

use afire::prelude::*;
use parking_lot::Mutex;
use rusqlite::Connection;

mod common;
mod routes;
mod session;

pub struct App {
    database: Mutex<Connection>,
    sessions: Mutex<Vec<session::Session>>,
    salt: Vec<u8>,
}

fn main() {
    let mut conn = Connection::open("data/data.db").unwrap();
    conn.pragma_update(None, "journal_mode", "WAL").unwrap();
    conn.pragma_update(None, "synchronous", "NORMAL").unwrap();

    let trans = conn.transaction().unwrap();

    // Init tables
    for i in [
        include_str!("sql/create_barks.sql"),
        include_str!("sql/create_users.sql"),
    ] {
        trans.execute(i, []).unwrap();
    }
    trans.commit().unwrap();

    let app = Arc::new(App {
        database: Mutex::new(conn),
        sessions: Mutex::new(Vec::new()),
        salt: "Pepper".as_bytes().to_vec(),
    });

    let mut server = Server::new("localhost", 8080);
    routes::attatch(&mut server, app);

    server.start().unwrap();
}
