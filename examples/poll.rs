use discord_webhook2::message::Message;
use discord_webhook2::webhook::DiscordWebhook;

#[tokio::main]
pub async fn main() {
    let webhook = DiscordWebhook::new(env!("DISCORD_WEBHOOK_URL")).unwrap();

    webhook
        .send(&Message::new(|message| {
            message.poll(
                |poll| poll.text("Yippie A poll!!"),
                |poll| {
                    poll.answer(|answer| answer.text("Answer #1"))
                        .answer(|answer| answer.text("Answer #2"))
                        .answer(|answer| answer.text("Answer #3"))
                },
            )
        }))
        .await
        .unwrap();
}
