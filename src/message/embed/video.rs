use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmbedVideo {
    pub url: String,
    #[serde(rename = "proxy_url")]
    pub url_proxy: Option<String>,
    pub width: Option<u16>,
    pub height: Option<u16>,
}