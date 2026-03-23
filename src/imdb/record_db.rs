use std::collections::HashMap;

use crate::generated::proto_server::Vector3;

#[derive(Debug, Clone)]
pub struct PlayerRecord {
    pub user_id: u64,

    pub user_name: String,

    pub last_zone_id: u64,
    pub last_position: Option<Vector3>,
}

#[derive(Debug, Default)]
pub struct RecordImdb {
    records: HashMap<u64, PlayerRecord>,
}

impl PlayerRecord {
    pub fn new(
        user_id: u64,
        user_name: String,
        last_zone_id: u64,
        last_position: Option<Vector3>,
    ) -> Self {
        Self {
            user_id,
            user_name,
            last_zone_id,
            last_position,
        }
    }
}

impl RecordImdb {
    pub fn load_player_record(&self, user_id: u64) -> Option<PlayerRecord> {
        self.records.get(&user_id).cloned()
    }

    pub fn save_player_record(&mut self, record: PlayerRecord) -> Option<()> {
        let Some(player) = self.records.get_mut(&record.user_id) else {
            return None;
        };

        *player = record;
        Some(())
    }

    pub fn create_player_record(&mut self, user_id: u64) -> Option<()> {
        if self.records.contains_key(&user_id) {
            return None;
        }

        let record = PlayerRecord::new(user_id, String::new(), 0, None);
        self.records.insert(user_id, record);
        Some(())
    }
}
