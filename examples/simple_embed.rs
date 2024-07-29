use discord_webhook2::message::Message;
use discord_webhook2::webhook::DiscordWebhook;

#[tokio::main]
async fn main() {
    let webhook = DiscordWebhook::new(env!("DISCORD_WEBHOOK_URL")).unwrap();

    webhook
        .send(&Message::new(|message| {
            message.embed(|embed| {
                embed
                    .title("Embed Title")
                    .description("description")
                    .url("https://example.com")
            })
        }))
        .await
        .unwrap();
}
