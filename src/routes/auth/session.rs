use afire::prelude::*;
use serde_json::Value;

use crate::{App, Arc};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::POST, "/auth/session", move |req| {
        // Get session id
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
            None => return Response::new().text("No token supplied"),
        }
        .to_owned();

        // Get session
        let session = match app.sessions.lock().iter().find(|x| x.session_id == session) {
            Some(i) => i,
            None => return Response::new().status(400).text("Invalid Session"),
        }
        .to_owned();

        if session.created.elapsed().as_secs() > app.config.session_timeout {
            app.sessions
                .lock()
                .retain(|x| x.session_id != session.user_id);
            return Response::new().status(400).text("Session expired");
        }

        // Send response / Remove cookie
        Response::new()
            .text(format!(
                r#"{{"session": "{}", "userId": "{}", "remainingTime": {}}}"#,
                session.session_id,
                session.user_id,
                app.config.session_timeout - session.created.elapsed().as_secs()
            ))
            .content(Content::JSON)
    });
}
