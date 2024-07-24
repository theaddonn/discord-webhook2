use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedFooter {
    pub text: String,
    #[serde(rename = "icon_url")]
    pub url_icon: Option<String>,
    #[serde(rename = "proxy_icon_url")]
    pub url_icon_proxy: Option<String>,
}
