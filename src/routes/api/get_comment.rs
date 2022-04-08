use afire::prelude::*;

use crate::{common::safe_json, App, Arc, Server};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/api/comments/{id}", move |req| {
        let id = req.path_param("id").unwrap();

        // Add message to database
        let out = {
            let db = app.database.lock();
            let mut stmp = db
                .prepare(include_str!("../../sql/query_comments.sql"))
                .unwrap();
            stmp.query_map([id], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            })
            .unwrap()
            .map(|x| x.unwrap())
            .map(|x: (String, String, String, String, u64)| {
                format!(
                    r#"{{"id": "{}", "content": "{}", "date": {}, "author": {{"id": "{}", "username": "{}"}}}}"#,
                    x.0, safe_json(&x.2), x.4, x.1, safe_json(&x.3)
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
