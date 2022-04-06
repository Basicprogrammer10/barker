use afire::prelude::*;
use rusqlite::{params, Error};
use serde_json::Value;

use crate::{App, Arc, Server};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::POST, "/api/get", move |req| {
        // Get bark id
        let body = req.body_string().unwrap();
        let json: Value = match serde_json::from_str(&body) {
            Ok(i) => i,
            Err(e) => {
                return Response::new()
                    .status(400)
                    .text(format!("Error parsing JSON: {}", e))
            }
        };
        let bark_id = match json.get("id") {
            Some(i) => i.as_str().unwrap(),
            None => {
                return Response::new()
                    .status(400)
                    .text("You must supply the id to a bark")
            }
        };

        // Get bark
        let (content, date, author_id, author_username): (String, u64, String, String) = match app.database.lock().query_row("SELECT content, barks.date, users.id, users.username FROM barks JOIN users ON barks.author_id = users.id WHERE barks.id = ?", params![bark_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))) {
            Ok(i) => i,
            Err(Error::QueryReturnedNoRows) => return Response::new().text(r#"{"error": "Bark not found"}"#).content(Content::JSON),
            e => e.unwrap()
        };

        // Send response
        Response::new()
            .text(format!(r#"{{"content": "{}", "date": {}, "author": {{"id": "{}", "username": "{}"}}}}"#, content, date, author_id, author_username))
            .content(Content::JSON)
    });
}
