use crate::{App, Arc, Server};

mod get_bark;
mod new_bark;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    get_bark::attatch(server, app.clone());
    new_bark::attatch(server, app);
}
