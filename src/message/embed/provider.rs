use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

impl EmbedProvider {
    pub fn new() -> Self {
        Self {
            name: None,
            url: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}
