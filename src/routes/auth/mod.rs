use crate::{App, Arc, Server};

mod login;
mod logout;
mod session;
mod signup;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    login::attatch(server, app.clone());
    logout::attatch(server, app.clone());
    session::attatch(server, app.clone());
    signup::attatch(server, app);
}
