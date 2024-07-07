pub mod video;
mod author;
mod footer;
mod image;
mod thumbnail;
mod provider;

use std::time::SystemTime;
use iso8061_timestamp::Timestamp;
use serde::Serialize;
use crate::embed::video::EmbedVideo;

#[derive(Serialize)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<EmbedAuthor>,
    pub url: Option<String>,
    pub color: Option<u32>,
    pub timestamp: Option<Timestamp>,
    pub video: Option<EmbedVideo>
}
