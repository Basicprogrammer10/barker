use crate::{App, Arc, Server};

mod auth;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    auth::attatch(server, app);
}
