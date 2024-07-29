use serde::{Deserialize, Serialize};

use crate::id::DiscordID;
use crate::message::embed::Embed;
use crate::message::flags::MessageFlags;
use crate::message::poll::create::PollCreate;
use crate::message::poll::media::PollMedia;

pub mod embed;
pub mod emoji;
pub mod flags;
pub mod poll;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: Option<DiscordID>,
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

    /// Well, a pool
    pub poll: Option<PollCreate>,

    /// Message flags combined as a bitfield
    /// (only `suppress_embeds` and `suppress_notifications` can be set can be set by webhooks)
    pub flags: Option<MessageFlags>,
}

impl Message {
    pub fn new<Func>(function: Func) -> Self
    where
        Func: FnOnce(Message) -> Message,
    {
        function(Self {
            id: None,
            content: None,
            username: None,
            avatar_url: None,
            tts: None,
            embeds: None,
            poll: None,
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
        Func: FnOnce(Embed) -> Embed,
    {
        let embed = function(Embed::new());

        match &mut self.embeds {
            None => self.embeds = Some(vec![embed]),
            Some(ref mut vec) => vec.push(embed),
        };
        self
    }

    pub fn poll<Func1, Func2>(mut self, function1: Func1, function2: Func2) -> Self
    where
        Func1: FnOnce(PollMedia) -> PollMedia,
        Func2: FnOnce(PollCreate) -> PollCreate,
    {
        self.poll = Some(function2(PollCreate::new(function1(PollMedia::new()))));
        self
    }
}
