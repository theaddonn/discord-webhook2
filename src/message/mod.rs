use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::embed::Embed;

pub mod content;
pub mod poll;
pub mod embed;

#[derive(Serialize)]
pub struct Message {
    pub content: Option<String>,
    /// Override the default username of the webhook
    #[serde(rename = "username")]
    pub override_username: Option<String>,
    /// Override the default avatar of the webhook
    #[serde(rename = "avatar_url")]
    pub override_avatar_url: Option<String>,
    /// Enables Text-To-Speech
    pub tts: Option<bool>,

    pub embeds: Option<Vec<Embed>>,
}
