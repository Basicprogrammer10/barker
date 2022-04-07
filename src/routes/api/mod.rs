use crate::{App, Arc, Server};

mod delete_bark;
mod get_bark;
mod new_bark;
mod recent_barks;
mod set_like;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    delete_bark::attatch(server, app.clone());
    get_bark::attatch(server, app.clone());
    new_bark::attatch(server, app.clone());
    set_like::attatch(server, app.clone());
    recent_barks::attatch(server, app);
}
