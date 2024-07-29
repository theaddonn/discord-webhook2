use serde::{Deserialize, Serialize};

use crate::id::DiscordID;
use crate::message::poll::media::PollMedia;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollAnswer {
    /// The ID of the answer
    id: Option<DiscordID>,
    /// The data of the answer
    poll_media: PollMedia,
}

impl PollAnswer {
    pub fn new(media: PollMedia) -> Self {
        Self {
            id: None,
            poll_media: media,
        }
    }
}
