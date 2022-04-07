use crate::{App, Arc, Server};

mod api;
mod auth;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    api::attatch(server, app.clone());
    auth::attatch(server, app.clone());
}
