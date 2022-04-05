use afire::prelude::*;
use afire::SetCookie;

use crate::{App, Arc};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::POST, "/auth/logout", move |req| {
        // Get session id from cookie
        let session = &match req.cookies.iter().find(|x| x.name == "session") {
            Some(i) => i,
            None => return Response::new().text("You arent logged in!"),
        }
        .value
        .to_owned();

        // Delete session
        app.sessions.lock().retain(|x| x.session_id != *session);

        // Send response / Remove cookie
        Response::new()
            .text("Logged out")
            .cookie(SetCookie::new("session", "").max_age(0))
    });
}
