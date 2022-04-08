use afire::prelude::*;
use rusqlite::params;
use serde_json::Value;

use crate::{App, Arc, Server};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::POST, "/api/deleteComment", move |req| {
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
        let token = match json.get("token") {
            Some(i) => i.as_str().unwrap(),
            None => {
                return Response::new()
                    .status(400)
                    .text(r#"{"error": "No token suplied"}"#)
                    .content(Content::JSON)
            }
        };
        let comment_id = match json.get("id") {
            Some(i) => i.as_str().unwrap(),
            None => {
                return Response::new()
                    .status(400)
                    .text(r#"{"error": "No comment id suplied"}"#)
                    .content(Content::JSON)
            }
        };

        // Get and Valadate session
        let session = match app.sessions.lock().iter().find(|x| x.session_id == *token) {
            Some(i) => i.to_owned(),
            None => {
                return Response::new()
                    .status(400)
                    .text(r#"{"error": "Invalid session"}"#)
                    .content(Content::JSON)
            }
        };

        if session.created.elapsed().as_secs() > app.config.session_timeout {
            app.sessions
                .lock()
                .retain(|x| x.session_id != session.user_id);
            return Response::new()
                .status(400)
                .text(r#"{"error": "Session expired"}"#)
                .content(Content::JSON);
        }

        // Verify user is owner of comment
        let access: u64 = app
            .database
            .lock()
            .query_row(
                "SELECT COUNT(*) FROM comments WHERE id = ? AND user_id = ?;",
                params![comment_id, session.user_id],
                |row| row.get(0),
            )
            .unwrap();

        if access == 0 {
            return Response::new()
                .status(400)
                .text(r#"{"error": "You dont own this comment!"}"#)
                .content(Content::JSON);
        }

        app.database
            .lock()
            .execute(
                include_str!("../../sql/execute_delete_comment.sql"),
                [comment_id],
            )
            .unwrap();

        Response::new()
            .text(r#"{"delete": "success"}"#)
            .content(Content::JSON)
    });
}
