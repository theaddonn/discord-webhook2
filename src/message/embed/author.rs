use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    #[serde(rename = "icon_url")]
    pub url_icon: Option<String>,
    #[serde(rename = "proxy_icon_url")]
    pub url_icon_proxy: Option<String>,
}
