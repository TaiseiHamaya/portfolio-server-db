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
        let result: Option<User> = match self
            .db_client
            .query()
            .table_name("UserSession")
            .key_condition_expression("#pk = :pk_val")
            .expression_attribute_names("#pk", "session_id")
            .expression_attribute_values(
                ":pk_val",
                aws_sdk_dynamodb::types::AttributeValue::B(session_id.to_vec().into()),
            )
            .send()
            .await
        {
            Ok(output) => match output.count {
                1 => output
                    .items
                    .and_then(|items| items.into_iter().next())
                    .and_then(|item| serde_dynamo::from_item(item).ok()),
                0 => {
                    log::info!(
                        "INFO | DB | User | auth_user | No user found for session_id: {:?}",
                        session_id
                    );
                    None
                }
                _ => {
                    log::error!(
                        "ERR | DB | User | auth_user | Multiple users found for session_id: {:?}",
                        session_id
                    );
                    None
                }
            },
            Err(e) => {
                log::error!("ERR | DB | User | auth_user | Info: {:?}", e);
                return None;
            }
        };
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
