use afire::prelude::*;
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{session::Session, App, Arc};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::POST, "/auth/login", move |req| {
        // Get username & password from request
        let body = req.body_string().unwrap();
        let json: Value = match serde_json::from_str(&body) {
            Ok(i) => i,
            Err(e) => {
                return Response::new()
                    .status(400)
                    .text(format!("Error parsing JSON: {}", e))
            }
        };
        let username = match json.get("username") {
            Some(i) => i.as_str().unwrap(),
            None => {
                return Response::new()
                    .status(400)
                    .text("You must supply a username")
            }
        };
        let password = match json.get("password") {
            Some(i) => i.as_str().unwrap(),
            None => {
                return Response::new()
                    .status(400)
                    .text("You must supply a username")
            }
        };

        // Get from database
        let (id, password_hash): (String, String) = match app.database.lock().query_row(
            "SELECT id, password FROM users WHERE username = ?",
            [username],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ) {
            Ok(i) => i,
            Err(_) => {
                return Response::new()
                    .status(400)
                    .text(r#"{"error": "Incorrect username"}"#)
                    .content(Content::JSON)
            }
        };

        // Valadate password
        let mut hasher = Sha256::new();
        let mut pass = password.as_bytes().to_vec();
        pass.extend(app.config.password_salt.clone());
        hasher.update(pass);
        let hash = format!("{:x}", hasher.finalize());

        if hash != password_hash {
            return Response::new()
                .status(400)
                .text(r#"{"error": "Incorrect password"}"#)
                .content(Content::JSON);
        }

        // Remove any old sessions
        app.sessions.lock().retain(|x| x.user_id != id);

        // Create session
        let session = Session::new(id.to_owned());
        let ses_id = session.session_id.to_owned();
        app.sessions.lock().push(session);
        println!("🎄 User Login [{}, {}]", username, ses_id);

        // Send response with cookie
        Response::new()
            .text(format!(r#"{{"token": "{}", "userId": "{}"}}"#, ses_id, id))
            .content(Content::JSON)
    });
}
