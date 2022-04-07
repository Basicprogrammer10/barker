use afire::prelude::*;

use crate::{common::safe_json, App, Arc, Server};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/api/recent", move |req| {
        let count = match req.query.get("count") {
            Some(i) => i.parse::<usize>().unwrap(),
            None => 25
        };

        if count > 1000 {
            return Response::new().status(400).text(r#"{"error": "Count too high (>1000)"}"#).content(Content::JSON);
        }

        // Get recent barks
        let out = {
            let db = app.database.lock();
            let mut stmt = db.prepare(include_str!("../../sql/query_recent_barks.sql")).unwrap();
            stmt.query_map([count], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?))).unwrap().into_iter().map(|x| x.unwrap()).map(|x: (String, u64, String, String, String, u64)| format!(r#"{{"content": "{}", "likes": {}, "date": {}, "id": "{}", "author": {{"id": "{}", "username": "{}"}}}}"#, safe_json(&x.0), x.5, x.1, x.2, x.3, safe_json(&x.4))).collect::<Vec<String>>().join(", ")
        };

        // Send response
        Response::new().text(format!("[{}]", out)).content(Content::JSON)
    });
}
