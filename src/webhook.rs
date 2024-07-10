use std::collections::BTreeMap;

use reqwest::multipart::{Form, Part};
use reqwest::{Client, Url};

use crate::error::DiscordWebhookError;
use crate::id::MessageID;
use crate::message::Message;
use crate::DiscordWebhookError::ReqwestError;

pub struct DiscordWebhook {
    client: Client,
    url: Url,
}

impl DiscordWebhook {
    pub fn new(url: impl Into<String>) -> Result<Self, DiscordWebhookError> {
        let url = url.into();
        let url = Url::parse(url.as_str()).map_err(|e| DiscordWebhookError::UrlParseError(e))?;

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

    /// Send the given Message object. Returns the ID of the send message as a result
    pub async fn send(&self, message: &Message) -> Result<MessageID, DiscordWebhookError> {
        let send_result = self
            .client
            .post(self.url.join("?wait=true").unwrap().clone())
            .json(message)
            .send()
            .await;

        let response = send_result.map_err(|e| DiscordWebhookError::ReqwestError(e))?;

        match response.status().is_success() {
            true => {
                let posted_message: Message = response
                    .json::<Message>()
                    .await
                    .map_err(|e| ReqwestError(e))?;

                match posted_message.id {
                    None => Err(DiscordWebhookError::FormatError(String::from(
                        "Missing field `id` in response",
                    ))),
                    Some(v) => Ok(v),
                }
            }
            false => Err(DiscordWebhookError::FormatError(
                response.text().await.unwrap().to_string(),
            )),
        }
    }

    /// Send the given Message object. Returns the ID of the send message as a result
    pub async fn send_with_files(
        &self,
        message: &Message,
        files_entries: BTreeMap<String, Vec<u8>>,
    ) -> Result<MessageID, DiscordWebhookError> {
        let mut form = Form::new().text("payload_json", serde_json::to_string(message).unwrap());

        for (i, (name, data)) in files_entries.into_iter().enumerate() {
            let mut part = Part::bytes(data);

            part = part.file_name(name);

            form = form.part(format!("files[{i}]"), part);
        }

        let send_result = self
            .client
            .post(self.url.join("?wait=true").unwrap().clone())
            .multipart(form)
            .send()
            .await;

        let response = send_result.map_err(|e| DiscordWebhookError::ReqwestError(e))?;

        match response.status().is_success() {
            true => {
                let posted_message: Message = response
                    .json::<Message>()
                    .await
                    .map_err(|e| ReqwestError(e))?;

                match posted_message.id {
                    None => Err(DiscordWebhookError::FormatError(String::from(
                        "Missing field `id` in response",
                    ))),
                    Some(v) => Ok(v),
                }
            }
            false => Err(DiscordWebhookError::FormatError(
                response.text().await.unwrap().to_string(),
            )),
        }
    }

    pub async fn edit(
        &self,
        message_id: MessageID,
        message: &Message,
    ) -> Result<MessageID, DiscordWebhookError> {
        let url = Url::parse(&format!("{}/", self.url.as_str())).unwrap();

        let send_result = self
            .client
            .patch(
                url.join(format!("messages/{}?wait=true", message_id.0).as_str())
                    .unwrap()
                    .clone(),
            )
            .json(message)
            .send()
            .await;

        let response = send_result.map_err(|e| DiscordWebhookError::ReqwestError(e))?;

        match response.status().is_success() {
            true => {
                let posted_message: Message = response
                    .json::<Message>()
                    .await
                    .map_err(|e| ReqwestError(e))?;

                match posted_message.id {
                    None => Err(DiscordWebhookError::FormatError(String::from(
                        "Missing field `id` in response",
                    ))),
                    Some(v) => Ok(v),
                }
            }
            false => Err(DiscordWebhookError::FormatError(
                response.text().await.unwrap().to_string(),
            )),
        }
    }

    pub async fn delete(&self, message_id: MessageID) -> Result<(), DiscordWebhookError> {
        let url = Url::parse(&format!("{}/", self.url.as_str())).unwrap();

        let send_result = self
            .client
            .delete(
                url.join(format!("messages/{}", message_id.0).as_str())
                    .unwrap()
                    .clone(),
            )
            .send()
            .await;

        send_result.map_err(|e| DiscordWebhookError::ReqwestError(e))?;

        Ok(())
    }
}
