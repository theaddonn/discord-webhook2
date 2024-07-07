use serde::Serialize;

#[derive(Serialize)]
pub struct EmbedProvider {
    name: Option<String>,
    url: Option<String>,
}