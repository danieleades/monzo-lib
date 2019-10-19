use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshTokens {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) refresh_token: String,
}

impl RefreshTokens {
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            refresh_token: refresh_token.into(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RefreshResponse {
    pub access_token: String,
    pub client_id: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub token_type: String,
    pub user_id: String,
}

impl RefreshResponse {
    pub fn into_tokens(self) -> (String, String) {
        (self.access_token, self.refresh_token)
    }
}

pub struct RefreshRequest {}
