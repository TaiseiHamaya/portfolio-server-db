use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::generated::proto_server::{SessionId, Vector3};

pub struct Record {
    session_id: SessionId,

    user_name: String,

    last_zone_id: u64,
    last_position: Option<Vector3>,
}

pub struct RecordImdb {
    pub records: Arc<RwLock<HashMap<SessionId, Record>>>,
}

impl RecordImdb {
    pub fn new() -> Self {
        Self {
            records: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
