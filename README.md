# discord-webhook2

_A simple rust library to interact with discord webhooks._

Why use this one over the others?

It is the only one to support:  
- uploading files/extra data
- handling message ids
- editing and deleting messages

and more...

### Example

Hello world:

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

More can be found in the [examples' directory](https://github.com/Adrian8115/discord-webhook2/tree/main/examples).

### Contributing

Contributing is always appreciated.
Feel free to create issues or open a pull request at any time.
