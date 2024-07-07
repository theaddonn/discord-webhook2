pub mod video;
pub mod author;
pub mod footer;
pub mod image;
pub mod thumbnail;
pub mod provider;
pub mod field;

use std::time::SystemTime;
use iso8061_timestamp::Timestamp;
use serde::Serialize;
use crate::embed::author::EmbedAuthor;
use crate::embed::field::EmbedField;
use crate::embed::footer::EmbedFooter;
use crate::embed::image::EmbedImage;
use crate::embed::provider::EmbedProvider;
use crate::embed::thumbnail::EmbedThumbnail;
use crate::embed::video::EmbedVideo;

#[derive(Serialize)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<Timestamp>,
    pub color: Option<u32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>
}
