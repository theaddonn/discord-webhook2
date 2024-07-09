use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}
