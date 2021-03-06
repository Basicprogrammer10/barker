use afire::prelude::*;
use rusqlite::{params, Error};
use serde_json::Value;

use crate::{App, Arc, Server};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::POST, "/api/delete", move |req| {
        // Get user session
        let body = req.body_string().unwrap();
        let json: Value = match serde_json::from_str(&body) {
            Ok(i) => i,
            Err(e) => {
                return Response::new()
                    .status(400)
                    .text(format!(
                        r#"{{"error":  "Error parsing JSON", "details": "{}"}}"#,
                        e
                    ))
                    .content(Content::JSON)
            }
        };
        let session = match json.get("token") {
            Some(i) => i.as_str().unwrap(),
            None => {
                return Response::new()
                    .status(400)
                    .text(r#"{"error": "No token defined"}"#)
                    .content(Content::JSON);
            }
        };
        let message = match json.get("message") {
            Some(i) => i.as_str().unwrap(),
            None => {
                return Response::new()
                    .status(400)
                    .text(r#"{"error": "No message defined"}"#)
                    .content(Content::JSON);
            }
        };

        // Get used id from session
        let session = match app
            .sessions
            .lock()
            .iter()
            .find(|x| x.session_id == *session)
        {
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

        // Check if session belongs to the message author
        let access: u64 = app
            .database
            .lock()
            .query_row(
                "SELECT COUNT(*) FROM barks WHERE id = ? AND author_id = ?;",
                params![message, session.user_id],
                |row| row.get(0),
            )
            .unwrap();

        if access == 0 {
            return Response::new()
                .status(400)
                .text(r#"{"error": "You dont own this bark!"}"#)
                .content(Content::JSON);
        }

        // Tru to set message to deleated
        match app
            .database
            .lock()
            .execute("UPDATE barks SET deleted = true WHERE id = ?", [message])
        {
            Ok(_) => {}
            Err(Error::SqliteFailure(_, _)) => {
                return Response::new()
                    .status(400)
                    .text(r#"{"error": "Message not found"}"#)
                    .content(Content::JSON)
            }
            e => {
                e.unwrap();
            }
        }
        println!("???? Deleted bark [{}]", message);

        // Send response
        Response::new()
            .text(r#"{"delete": "success"}"#)
            .content(Content::JSON)
    });
}
