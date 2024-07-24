use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    #[serde(rename = "icon_url")]
    pub url_icon: Option<String>,
    #[serde(rename = "proxy_icon_url")]
    pub url_icon_proxy: Option<String>,
}

impl EmbedAuthor {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            url: None,
            url_icon: None,
            url_icon_proxy: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
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
