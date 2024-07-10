use std::time::Duration;

use tokio::time::sleep;

use discord_webhook2::{DiscordWebhook, Message};

#[tokio::main]
async fn main() {
    let webhook = DiscordWebhook::new(env!("DISCORD_WEBHOOK_URL")).unwrap();

    let message_id = webhook
        .send(&Message::new(|message| {
            message.content("I'll delete this message again")
        }))
        .await
        .unwrap();

    sleep(Duration::from_secs(3)).await;

    webhook.delete(message_id).await.unwrap();
}
