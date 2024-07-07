use std::time::SystemTime;
use discord_webhook2::{DiscordWebhook, Message, message};

#[tokio::main]
async fn main() {
    let webhook = DiscordWebhook::new(env!("DISCORD_WEBHOOK_URL")).unwrap();

    let time = SystemTime::now();

    let message = Message {
        content: Some("https://www.youtube.com/watch?v=GFLb5h2O2Ww".to_string()),
        override_username: None,
        override_avatar_url: None,
        tts: None,
        embeds: None,
    };

    webhook.send(&message).await;

}