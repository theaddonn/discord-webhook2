use serde::Serialize;

#[derive(Serialize)]
pub struct EmbedImage {
    pub url: String,
    #[serde(rename = "proxy_url")]
    pub url_proxy: Option<String>,
    pub width: Option<u16>,
    pub height: Option<u16>,
}
