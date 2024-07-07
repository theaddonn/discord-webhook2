use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug, Clone)]
pub enum DiscordWebhookError {
    #[error("URL Parse Error: {0}")]
    UrlParseError(ParseError)
}
