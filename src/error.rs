use reqwest::Error as ReqwestError;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum DiscordWebhookError {
    #[error("URL Parse Error: {0}")]
    UrlParseError(#[from] ParseError),

    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] ReqwestError),

    #[error("Format Error: {0}")]
    FormatError(String),
}
