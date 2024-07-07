use reqwest::{Client, Error, Method, Request, RequestBuilder, Response, Url};
use crate::error::DiscordWebhookError;
use crate::message::Message;

pub struct DiscordWebhook {
    client: Client,
    url: Url,
}

impl DiscordWebhook {
    pub fn new(url: impl Into<String>) -> Result<Self, DiscordWebhookError> {
        let url = url.into();
        let url = Url::parse(url.as_str()).map_err(|e| { DiscordWebhookError::UrlParseError(e) })?;

        Ok(Self {
            client: Client::new(),
            url
        })
    }

    pub fn from_url(url: Url) -> Self {
        Self {
            client: Client::new(),
            url
        }
    }

    pub async fn send(&self, message: &Message) {
        let response = self.client
            .post(self.url.clone())
            .json(message)
            .send().await;

        println!("RESPONSE: {response:#?}");
    }
}
