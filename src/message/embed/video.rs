use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedVideo {
    pub url: String,
    #[serde(rename = "proxy_url")]
    pub url_proxy: Option<String>,
    pub width: Option<u16>,
    pub height: Option<u16>,
}

impl EmbedVideo {
    pub fn new() -> Self {
        Self {
            url: String::new(),
            url_proxy: None,
            width: None,
            height: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    pub fn url_proxy(mut self, url_proxy: impl Into<String>) -> Self {
        self.url_proxy = Some(url_proxy.into());
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }
}

impl Default for EmbedVideo {
    fn default() -> Self {
        Self::new()
    }
}
