use afire::prelude::*;

use crate::{App, Arc, Server};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/api/comment/{id}", move |req| {
        let id = req.path_param("id").unwrap();

        // Add message to database
        let out = {
            let db = app.database.lock();
            let mut stmp = db
                .prepare(include_str!("../../sql/query_comments.sql"))
                .unwrap();
            stmp.query_map([id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
                .unwrap()
                .map(|x| x.unwrap())
                .map(|x: (String, String, String, String)| {
                    format!(
                        r#"{{"id": "{}", "content": "{}", "author": {{"id": "{}", "username": "{}"}}}}"#,
                        x.0, x.2, x.1, x.3
                    )
                })
                .collect::<Vec<_>>()
                .join(",")
        };

        // Send response
        Response::new()
            .text(format!(r#"[{}]"#, out))
            .content(Content::JSON)
    });
}
