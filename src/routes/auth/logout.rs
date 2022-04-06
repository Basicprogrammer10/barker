use afire::prelude::*;
use serde_json::Value;

use crate::{App, Arc};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::POST, "/auth/logout", move |req| {
        // Get session id
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
                    .text(r#"{"error": "No token supplied"}"#)
                    .content(Content::JSON)
            }
        }
        .to_owned();

        // Delete session
        if !session.is_empty() {
            println!("🗑 User Logout [{}]", session);
            app.sessions.lock().retain(|x| x.session_id != *session);
        }

        // Send response / Remove cookie
        Response::new()
            .text(r#"{"logout": "sucess"}"#)
            .content(Content::JSON)
    });
}
