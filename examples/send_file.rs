use std::collections::BTreeMap;

use discord_webhook2::{DiscordWebhook, Message};

#[tokio::main]
async fn main() {
    let webhook = DiscordWebhook::new(env!("DISCORD_WEBHOOK_URL")).unwrap();

    let mut files = BTreeMap::new();

    files.insert(
        String::from("send_file.png"),
        Vec::from(include_bytes!("send_file.png")),
    );

    webhook
        .send_with_files(
            &Message::new(|message| {
                message.embed(|embed| embed.image(|image| image.url("attachment://send_file.png")))
            }),
            files,
        )
        .await
        .unwrap();
}
