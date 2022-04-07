use afire::prelude::*;
use rusqlite::params;
use serde_json::Value;

use crate::{App, Arc, Server};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::POST, "/api/like", move |req| {
        // Get user session
        let body = req.body_string().unwrap();
        let json: Value = match serde_json::from_str(&body) {
            Ok(i) => i,
            Err(e) => {
                return Response::new()
                    .status(400)
                    .text(format!("Error parsing JSON: {}", e))
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
        let state = match json.get("state") {
            Some(i) => i.as_bool().unwrap(),
            None => {
                return Response::new()
                    .status(400)
                    .text(r#"{"error": "No like state defined"}"#)
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

        // Valadate Message is real
        let real_msg: u64 = app
            .database
            .lock()
            .query_row(
                "SELECT COUNT(*) FROM barks WHERE id = ?",
                [message],
                |row| row.get(0),
            )
            .unwrap();

        if real_msg < 1 {
            return Response::new()
                .status(400)
                .text(format!(r#"{{"Error": "Message not found"}}"#))
                .content(Content::JSON);
        }

        // Update Database
        app.database
            .lock()
            .execute(
                if state {
                    include_str!("../../sql/execute_add_like.sql")
                } else {
                    include_str!("../../sql/execute_remove_like.sql")
                },
                params![message, session.user_id],
            )
            .unwrap();

        // Send response
        Response::new()
            .text(format!(r#"{{"like": "success"}}"#))
            .content(Content::JSON)
    });
}
