use afire::prelude::*;
use rusqlite::{params, Error};
use serde_json::Value;

use crate::{common::safe_json, App, Arc, Server};

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
        let (content, date, author_id, author_username, deleted): (String, u64, String, String, bool) = match app.database.lock().query_row("SELECT content, barks.date, users.id, users.username, deleted FROM barks JOIN users ON barks.author_id = users.id WHERE barks.id = ?", params![bark_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))) {
            Ok(i) => i,
            Err(Error::QueryReturnedNoRows) => return Response::new().text(r#"{"error": "Bark not found"}"#).content(Content::JSON),
            e => e.unwrap()
        };

        if deleted {
            return Response::new().text(r#"{"error": "Bark deleted"}"#).content(Content::JSON);
        }

        // Send response
        Response::new()
            .text(format!(r#"{{"content": "{}", "id": "{}", "date": {}, "author": {{"id": "{}", "username": "{}"}}}}"#, safe_json(&content), bark_id, date, author_id, safe_json(&author_username)))
            .content(Content::JSON)
    });
}
