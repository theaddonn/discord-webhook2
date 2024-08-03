use crate::message::emoji::Emoji;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollMedia {
    /// The text of the field
    text: Option<String>,
    /// The emoji of the field
    emoji: Option<Emoji>,
}

impl PollMedia {
    pub fn new() -> Self {
        Self {
            text: None,
            emoji: None,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn emoji<Func>(mut self, function: Func) -> Self
    where
        Func: FnOnce(Emoji) -> Emoji,
    {
        self.emoji = Some(function(Emoji::new()));
        self
    }
}

impl Default for PollMedia {
    fn default() -> Self {
        Self::new()
    }
}
