use crate::{App, Arc, Server};

mod add_comment;
mod delete_bark;
mod delete_comment;
mod get_bark;
mod get_comment;
mod new_bark;
mod recent_barks;
mod set_like;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    add_comment::attatch(server, app.clone());
    delete_bark::attatch(server, app.clone());
    delete_comment::attatch(server, app.clone());
    get_bark::attatch(server, app.clone());
    get_comment::attatch(server, app.clone());
    new_bark::attatch(server, app.clone());
    set_like::attatch(server, app.clone());
    recent_barks::attatch(server, app);
}
