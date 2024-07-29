use std::time::Duration;

use tokio::time::sleep;
use discord_webhook2::message::Message;
use discord_webhook2::webhook::DiscordWebhook;

#[tokio::main]
async fn main() {
    let webhook = DiscordWebhook::new(env!("DISCORD_WEBHOOK_URL")).unwrap();

    let message_id = webhook
        .send(&Message::new(|message| message.content("Original text")))
        .await
        .unwrap();

    sleep(Duration::from_secs(3)).await;

    webhook
        .edit(
            &message_id,
            &Message::new(|message| message.content("Edited text")),
        )
        .await
        .unwrap();
}
