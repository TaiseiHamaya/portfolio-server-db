use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::generated::proto_server::SessionId;

pub struct Session {
    session_id: SessionId,

    user_id: u64,
}

pub struct SessionImdb {
    pub sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
}

impl SessionImdb {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
