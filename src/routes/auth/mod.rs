use crate::{App, Arc, Server};

mod login;
mod signup;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    signup::attatch(server, app.clone());
    login::attatch(server, app.clone());
}
