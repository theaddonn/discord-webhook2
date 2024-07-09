use iso8061_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

use crate::embed::author::EmbedAuthor;
use crate::embed::field::EmbedField;
use crate::embed::footer::EmbedFooter;
use crate::embed::image::EmbedImage;
use crate::embed::provider::EmbedProvider;
use crate::embed::thumbnail::EmbedThumbnail;
use crate::embed::video::EmbedVideo;

pub mod video;
pub mod author;
pub mod footer;
pub mod image;
pub mod thumbnail;
pub mod provider;
pub mod field;

#[derive(Serialize, Deserialize)]
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
    pub fields: Option<Vec<EmbedField>>,
}

impl Embed {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            url: None,
            timestamp: None,
            color: None,
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}