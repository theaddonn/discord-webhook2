use std::collections::BTreeMap;
use std::io::Read;
use reqwest::{Client, Url};
use reqwest::multipart::{Form, Part};
use crate::DiscordWebhookError::ReqwestError;
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
            url,
        })
    }

    pub fn from_url(url: Url) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }

    pub async fn send(&self, message: &Message) -> Result<MessageID, DiscordWebhookError> {
        let send_result = self.client
            .post(self.url.join("?wait=true").unwrap().clone())
            .json(message)
            .send().await;

        let response = send_result.map_err(|e| DiscordWebhookError::ReqwestError(e))?;

        match response.status().is_success() {
            true => {
                let posted_message: Message = response.json::<Message>().await.map_err(|e| ReqwestError(e))?;

                match posted_message.id {
                    None => { Err(DiscordWebhookError::FormatError(String::from("Missing field `id` in response"))) }
                    Some(v) => { Ok(v) }
                }
            }
            false => {
                Err(DiscordWebhookError::FormatError(response.text().await.unwrap().to_string()))
            }
        }
    }

    pub async fn send_with_files(&self, message: &Message, files_entries: BTreeMap<String, Vec<u8>>) -> Result<MessageID, DiscordWebhookError> {
        let mut form = Form::new()
            .text("payload_json", serde_json::to_string(message).unwrap());

        for (i, (name, data)) in files_entries.into_iter().enumerate() {
            let mut part = Part::bytes(data);

            part = part.file_name(name);

            form = form.part(format!("files[{i}]"), part);
        }

        let send_result = self.client
            .post(self.url.join("?wait=true").unwrap().clone())
            .multipart(form)
            .send().await;

        let response = send_result.map_err(|e| DiscordWebhookError::ReqwestError(e))?;

        match response.status().is_success() {
            true => {
                let posted_message: Message = response.json::<Message>().await.map_err(|e| ReqwestError(e))?;

                match posted_message.id {
                    None => { Err(DiscordWebhookError::FormatError(String::from("Missing field `id` in response"))) }
                    Some(v) => { Ok(v) }
                }
            }
            false => {
                Err(DiscordWebhookError::FormatError(response.text().await.unwrap().to_string()))
            }
        }
    }
}
