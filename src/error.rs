use thiserror::Error;
use url::ParseError;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;

#[derive(Error, Debug)]
pub enum DiscordWebhookError {
    #[error("URL Parse Error: {0}")]
    UrlParseError(ParseError),

    #[error("Reqwest Error: {0}")]
    ReqwestError(ReqwestError),

    #[error("Format Error: {0}")]
    FormatError(String),
}
