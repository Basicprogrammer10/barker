use afire::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use rusqlite::params;
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{common::get_ip, App, Arc};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::POST, "/auth/signup", move |req| {
        // Get username from request
        let ip = get_ip(&req);
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
        let username = match json.get("username") {
            Some(i) => i.as_str().unwrap(),
            None => {
                return Response::new()
                    .status(400)
                    .text(r#"{"error": "You must supply a username"}"#)
                    .content(Content::JSON)
            }
        };
        let password = match json.get("password") {
            Some(i) => i.as_str().unwrap(),
            None => {
                return Response::new()
                    .status(400)
                    .text(r#"{"error": "You must supply a password"}"#)
                    .content(Content::JSON)
            }
        };

        // Check if it is inuse
        let fresh: usize = app
            .database
            .lock()
            .query_row(
                "SELECT COUNT(*) FROM users WHERE username = ?",
                [username],
                |row| row.get(0),
            )
            .unwrap();

        if fresh >= 1 {
            return Response::new().status(409).text(r#"{"error": "An account with that username already exists"}"#).content(Content::JSON);
        }

        // Hash + Salt Password
        let mut hasher = Sha256::new();
        let mut pass = password.as_bytes().to_vec();
        pass.extend(app.config.password_salt.clone());
        hasher.update(pass);
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);

        // Gen acc id
        let id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect::<String>();

        // Add to database
        println!("🧔 New user [{}, {}]", username, id);
        app.database
            .lock()
            .execute(
                "INSERT INTO users (id, username, password, ip, date) VALUES (?, ?, ?, ?, strftime('%s','now'))",
                params![id, username, hash, ip],
            )
            .unwrap();

        Response::new().status(201).text(format!(r#"{{"id": "{}"}}"#, id)).content(Content::JSON)
    });
}
