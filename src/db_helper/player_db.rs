use serde::{Deserialize, Serialize};

use crate::generated::proto_server::Vector3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRecord {
    pub user_id: String,

    pub user_name: String,

    pub last_zone_id: u64,
    pub last_position: Option<Vector3>,
}

impl PlayerRecord {
    pub fn new(
        user_id: u64,
        user_name: String,
        last_zone_id: u64,
        last_position: Option<Vector3>,
    ) -> Self {
        Self {
            user_id: user_id.to_string(),
            user_name,
            last_zone_id,
            last_position,
        }
    }
}

#[derive(Debug)]
pub struct PlayerDBHelper {
    db_client: aws_sdk_dynamodb::client::Client,
}

impl PlayerDBHelper {
    pub fn new(db_client: aws_sdk_dynamodb::client::Client) -> Self {
        Self { db_client }
    }
}

impl PlayerDBHelper {
    pub async fn load_player_record(&self, user_id: u64) -> Option<PlayerRecord> {
        self.db_client
            .get_item()
            .table_name("PlayerData")
            .key(
                "user_id",
                aws_sdk_dynamodb::types::AttributeValue::S(user_id.to_string()),
            )
            .send()
            .await
            .ok()
            .and_then(|output| output.item)
            .and_then(|item| serde_dynamo::from_item(item).ok())
    }

    pub async fn save_player_record(&self, record: PlayerRecord) -> Option<()> {
        let item = serde_dynamo::to_item(record).ok()?;
        self.db_client
            .put_item()
            .table_name("PlayerData")
            .set_item(Some(item))
            .send()
            .await
            .ok()?;
        Some(())
    }

    pub async fn create_player_record(&self, user_id: u64, username: String) -> Option<()> {
        let record = PlayerRecord::new(user_id, username, 0, None);
        let item = serde_dynamo::to_item(record).ok()?;
        self.db_client
            .put_item()
            .table_name("PlayerData")
            .set_item(Some(item))
            .send()
            .await
            .ok()?;
        Some(())
    }
}
