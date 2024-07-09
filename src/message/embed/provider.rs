use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}