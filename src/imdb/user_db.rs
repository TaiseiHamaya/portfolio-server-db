use std::collections::{BTreeMap, HashMap};

use rand::{TryRng, rngs};

use crate::generated::proto_server::SessionId;

#[derive(Debug, Clone, Default)]
pub struct User {
    pub user_id: u64,
    pub session_id: Option<SessionId>,
}

#[derive(Debug, Default)]
pub struct UserImdb {
    users: HashMap<u64, User>,
    user_id_counter: u64,
    index_session_id: BTreeMap<SessionId, u64>,

    rang: rngs::SysRng,
}

impl UserImdb {
    fn generate_session_id(&mut self) -> SessionId {
        SessionId {
            high: self.rang.try_next_u64().unwrap(),
            low: self.rang.try_next_u64().unwrap(),
        }
    }

    pub fn create_user(&mut self) -> Result<(u64, SessionId), Box<dyn std::error::Error>> {
        let user_id = self.user_id_counter;

        let generated_session_id = self.generate_session_id();
        if self.users.contains_key(&user_id)
            || self.index_session_id.contains_key(&generated_session_id)
        {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "User already exists",
            ))); // ユーザーIDが既に存在する場合はエラー
        }
        self.user_id_counter += 1;

        // セッションIDの生成
        let user = User {
            user_id,
            session_id: Some(generated_session_id),
        };

        // ユーザーを追加
        self.users.insert(user_id, user);
        self.index_session_id.insert(generated_session_id, user_id);

        // 生成されたセッションIDを返す
        Ok((user_id, generated_session_id))
    }

    pub fn auth_user(&mut self, session_id: &SessionId) -> Option<(u64, SessionId)> {
        let temp_session_id = self.generate_session_id();

        let user_id = self.index_session_id.get(session_id)?;
        let user = self.users.get_mut(user_id)?;
        if user.session_id.is_none() {
            user.session_id = Some(temp_session_id);
        }

        Some((user.user_id, user.session_id.unwrap()))
    }

    pub fn logout_user(&mut self, user_id: u64) -> Option<()> {
        let user = self.users.get_mut(&user_id)?;

        // セッションのクリア
        user.session_id = None;

        Some(())
    }
}
