use afire::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use rusqlite::params;
use serde_json::Value;

use crate::{common::get_ip, App, Arc, Server};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::POST, "/api/new", move |req| {
        // Get user session
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
            None => return Response::new()
                .status(400)
                .text(r#"{"error": "Invalid session"}"#)
                .content(Content::JSON),
        };

        // Valadate Session
        if session.created.elapsed().as_secs() > app.config.session_timeout {
            app.sessions.lock().retain(|x| x.session_id != session.user_id);
            return Response::new()
                .status(400)
                .text(r#"{"error": "Session expired"}"#)
                .content(Content::JSON);
        }

        // Valadate Message
        if message.len() > app.config.max_message_len && app.config.max_message_len != 0 {
            return Response::new().status(400).text(format!(
                r#"{{"error": "Message too long. Keep it under {} chars"}}"#,
                app.config.max_message_len
            ));
        }

        if message.is_empty() {
            return Response::new()
                .status(400)
                .text(r#"{"error": "Message body empty!"}"#)
                .content(Content::JSON);
        }

        let bark_id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();

        // Add message to database
        app.database.lock().execute("INSERT INTO barks (id, author_id, ip, content, deleted, date) VALUES (?, ?, ?, ?, false, strftime('%s','now'))", params![bark_id, session.user_id, get_ip(&req), message]).unwrap();
        println!("???? New Bark [{}, {}]", session.user_id, message);

        // Send response
        Response::new().text(format!(r#"{{"id": "{}"}}"#, bark_id)).content(Content::JSON)
    });
}
