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
                    .text(format!(r#"{{"error":  "Error parsing JSON", "details": "{}"}}"#, e))
                    .content(Content::JSON)
            }
        };
        let bark_id = match json.get("id") {
            Some(i) => i.as_str().unwrap(),
            None => {
                return Response::new()
                    .status(400)
                    .text(r#"{"error": "No message id suplied"}"#)
                    .content(Content::JSON)
            }
        };
        let token = json.get("token");

        // Get bark
        let (content, date, author_id, author_username, deleted, likes, likeing, comments): (
            String,
            u64,
            String,
            String,
            bool,
            u64,
            bool,
            u64
        ) = match if let Some(i) = token {
            // Get session
            let session = match app.sessions.lock().iter().find(|x| x.session_id == *i) {
                Some(i) => i.to_owned(),
                None => {
                    return Response::new()
                        .status(400)
                        .text(r#"{"error": "Invalid session"}"#)
                        .content(Content::JSON)
                }
            };

            // Valadate Session
            if session.created.elapsed().as_secs() > app.config.session_timeout {
                app.sessions
                    .lock()
                    .retain(|x| x.session_id != session.user_id);
                return Response::new()
                    .status(400)
                    .text(r#"{"error": "Session expired"}"#)
                    .content(Content::JSON);
            }

            app.database.lock().query_row(
                include_str!("../../sql/query_bark_loggedin.sql"),
                params![session.user_id, bark_id],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                        row.get(5)?,
                        row.get(6)?,
                        row.get(7)?,
                    ))
                },
            )
        } else {
            app.database.lock().query_row(
                include_str!("../../sql/query_bark.sql"),
                params![bark_id],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                        row.get(5)?,
                        false,
                        row.get(6)?,
                    ))
                },
            )
        } {
            Ok(i) => i,
            Err(Error::QueryReturnedNoRows) => {
                return Response::new()
                    .text(r#"{"error": "Bark not found"}"#)
                    .content(Content::JSON)
            }
            e => e.unwrap(),
        };

        if deleted {
            return Response::new()
                .text(r#"{"error": "Bark deleted"}"#)
                .content(Content::JSON);
        }

        // Send response
        Response::new()
            .text(format!(r#"{{"content": "{}", "id": "{}", "likes": {}, "likeing": {}, "date": {}, "comments": {}, "author": {{"id": "{}", "username": "{}"}}}}"#, safe_json(&content), bark_id, likes, likeing, date, comments, author_id, safe_json(&author_username)))
            .content(Content::JSON)
    });
}
