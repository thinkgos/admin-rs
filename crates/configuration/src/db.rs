use secrecy::SecretBox;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    // mysql://root:123456@127.0.0.1:3006/admin?ssl-mode=false
    url: SecretBox<String>,
}

impl Database {
    pub fn url(&self) -> &SecretBox<String> {
        &self.url
    }
}
