use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::embed::Embed;
use crate::flags::MessageFlags;
use crate::message::id::MessageID;

pub mod content;
pub mod poll;
pub mod embed;
pub mod flags;
pub mod id;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: Option<MessageID>,
    /// The message contents (up to 2000 characters)
    pub content: Option<String>,
    /// Override the default username of the webhook
    pub username: Option<String>,
    /// Override the default avatar of the webhook
    pub avatar_url: Option<String>,
    /// Enables Text-To-Speech
    pub tts: Option<bool>,

    /// Vector of up to 10 Embed objects
    pub embeds: Option<Vec<Embed>>,

    /// Message flags combined as a bitfield
    /// (only `suppress_embeds` and `suppress_notifications` can
    /// be set can be set by webhooks)
    pub flags: Option<MessageFlags>,
}

impl Message {
    pub fn new<Func>(function: Func) -> Self
    where
        Func: Fn(Message) -> Message,
    {
        function(Self {
            id: None,
            content: None,
            username: None,
            avatar_url: None,
            tts: None,
            embeds: None,
            flags: None,
        })
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn avatar_url(mut self, avatar_url: impl Into<String>) -> Self {
        self.avatar_url = Some(avatar_url.into());
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.tts = Some(tts);
        self
    }

    pub fn embed<Func>(mut self, function: Func) -> Self
    where
        Func: Fn(Embed) -> Embed,
    {
        let embed = function(Embed::new());

        match &mut self.embeds {
            None => { self.embeds = Some(vec![embed]) }
            Some(ref mut vec) => { vec.push(embed) }
        };
        self
    }
}
