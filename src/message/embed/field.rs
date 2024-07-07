use serde::Serialize;

#[derive(Serialize)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: Option<bool>,
}
