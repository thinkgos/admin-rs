use secrecy::Secret;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    // mysql://root:123456@127.0.0.1:3006/admin?ssl-mode=false
    url: Secret<String>,
}

impl Database {
    pub fn url(&self) -> &Secret<String> {
        &self.url
    }
}
