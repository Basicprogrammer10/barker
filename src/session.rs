use std::time::Instant;

use rand::{distributions::Alphanumeric, Rng};

#[derive(Debug, Clone)]
pub struct Session {
    pub created: Instant,
    pub user_id: String,
    pub session_id: String,
}

impl Session {
    pub fn new(id: String) -> Self {
        let session_id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();

        Self {
            user_id: id,
            created: Instant::now(),
            session_id,
        }
    }
}
