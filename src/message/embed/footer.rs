use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedFooter {
    pub text: String,
    #[serde(rename = "icon_url")]
    pub url_icon: Option<String>,
    #[serde(rename = "proxy_icon_url")]
    pub url_icon_proxy: Option<String>,
}

impl EmbedFooter {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            url_icon: None,
            url_icon_proxy: None,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn url_icon(mut self, url_icon: impl Into<String>) -> Self {
        self.url_icon = Some(url_icon.into());
        self
    }

    pub fn url_icon_proxy(mut self, url_icon_proxy: impl Into<String>) -> Self {
        self.url_icon_proxy = Some(url_icon_proxy.into());
        self
    }
}
