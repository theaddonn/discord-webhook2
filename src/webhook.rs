use reqwest::{Client, Url};

use crate::DiscordWebhookError::{FormatError, ReqwestError};
use crate::error::DiscordWebhookError;
use crate::id::MessageID;
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

    pub async fn send(&self, message: &Message) -> Result<MessageID, DiscordWebhookError> {
        let send_result = self.client
            .post(self.url.join("?wait=true").unwrap().clone())
            .json(message)
            .send().await;

        let response = send_result.map_err(|e| ReqwestError(e) )?;

        if response.status().is_success() {
            let posted_message: Message = response.json::<Message>().await.map_err(|e| ReqwestError(e) )?;

            match posted_message.id {
                None => { return Err(FormatError(String::from("Missing field `id` in response"))) }
                Some(v) => { return Ok(v) }
            }
        } else {
            return Err(FormatError(response.text().await.unwrap().to_string()))
        };
    }
}
