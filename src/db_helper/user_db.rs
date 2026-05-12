use rand::{Rng, rngs};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    #[serde(with = "serde_bytes")]
    pub session_id: [u8; 16],
}

#[derive(Debug)]
pub struct UserDBHelper {
    db_client: aws_sdk_dynamodb::client::Client,
}

impl UserDBHelper {
    pub fn new(db_client: aws_sdk_dynamodb::client::Client) -> Self {
        Self { db_client }
    }
}

impl UserDBHelper {
    fn generate_session_id(&self) -> [u8; 16] {
        let mut session_id = [0u8; 16];
        rngs::ThreadRng::default().fill_bytes(&mut session_id);
        session_id
    }

    pub async fn create_user(&self) -> Option<(u64, [u8; 16])> {
        // SessionIDの生成
        let user_id = rngs::ThreadRng::default().next_u64();
        let session_id = self.generate_session_id();
        let user = User {
            user_id: user_id.to_string(),
            session_id,
        };

        // ユーザーを追加
        let item = serde_dynamo::to_item(user).ok()?;
        match self
            .db_client
            .put_item()
            .table_name("UserSession")
            .set_item(Some(item))
            .condition_expression(
                "attribute_not_exists(user_id) AND attribute_not_exists(session_id)",
            )
            .send()
            .await
        {
            Ok(_) => {
                // 生成されたセッションIDを返す
                Some((user_id, session_id))
            }
            Err(e) => {
                log::error!("ERR | DB | User | create_user | Info: {}", e);
                None
            }
        }
    }

    pub async fn auth_user(&self, session_id: [u8; 16]) -> Option<(u64, [u8; 16])> {
        let mut key = std::collections::HashMap::new();
        key.insert("session_id", serde_bytes::Bytes::new(&session_id));
        let item = serde_dynamo::to_item(key).ok()?;
        let result: Option<User> = self
            .db_client
            .get_item()
            .table_name("UserSession")
            .set_key(Some(item))
            .send()
            .await
            .ok()
            .and_then(|output| output.item)
            .and_then(|item| serde_dynamo::from_item(item).ok());
        result.map(|user| {
            let user_id = user.user_id.parse::<u64>().unwrap_or_default();
            let session_id = user.session_id;
            (user_id, session_id)
        })
    }

    pub async fn logout_user(&self, _user_id: u64) -> Option<()> {
        Some(())
    }
}
