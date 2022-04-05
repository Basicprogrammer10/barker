use crate::{App, Arc, Server};

mod new_bark;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    new_bark::attatch(server, app.clone());
}
