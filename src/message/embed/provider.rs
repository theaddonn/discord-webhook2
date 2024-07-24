use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}
