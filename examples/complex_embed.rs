use iso8061_timestamp::Timestamp;
use discord_webhook2::{DiscordWebhook, Message};

#[tokio::main]
async fn main() {
    let webhook = DiscordWebhook::new(env!("DISCORD_WEBHOOK_URL")).unwrap();

    webhook
        .send(&Message::new(|message| message
            .embed(|embed| embed
                .title("Embed Title")
                .description("description")
                .url("https://example.com")
                .footer(|footer| footer
                    .text("Footer text")
                )
                .author(|author| author
                    .name("Author name")
                    .url("https://example.com")
                )
                .field(|field| field
                    .name("Field name 1")
                    .value("Value 1")
                )
                .field(|field| field
                    .name("Field name 2")
                    .inline(true)
                )
                .field(|field| field
                    .value("Value 3")
                )
                .color(0x00BBFF)
                .timestamp(Timestamp::now_utc())
            )
        ))
        .await
        .unwrap();
}
