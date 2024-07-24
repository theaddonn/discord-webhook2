# discord-webhook2

_A simple rust library to interact with discord webhooks._

Why use this one over the others?

While there are multiple other libraries,
most of them are either unmaintained or lack certain features...

Supports:
- [X] Embeds
- [X] Handling message ids
- [X] Uploading files/extra data
- [X] Sending, getting, editing and deleting messages

### Example

A basic "Hello World" using `discord-webhook2`:

```rust
use discord_webhook2::{DiscordWebhook, Message};

#[tokio::main]
async fn main() {
    let webhook = DiscordWebhook::new("discord webhook url").unwrap();

    webhook.send(&Message::new(|message| message
        .content("Hello World!")
    )).await.unwrap();
}
```

More examples can be found in the examples directory [discord-webhook2/examples](https://github.com/Adrian8115/discord-webhook2/tree/main/examples).

### Contributing

Contributing is always appreciated.
Feel free to create issues or open pull requests at any time.
