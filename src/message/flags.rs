use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct MessageFlags {
    /// This message has been published to subscribed channels (via Channel Following)
    crossposted: bool,
    /// This message originated from a message in another channel (via Channel Following)
    is_crosspost: bool,
    /// Do not include any embeds when serializing this message
    suppress_embeds: bool,
    /// The source message for this crosspost has been deleted (via Channel Following)
    source_message_deleted: bool,
    /// This message came from the urgent message system
    urgent: bool,
    /// This message has an associated thread, with the same id as the message
    has_thread: bool,
    /// This message is only visible to the user who invoked the Interaction
    ephemeral: bool,
    /// This message is an Interaction Response and the bot is "thinking"
    loading: bool,
    /// This message failed to mention some roles and add their members to the thread
    failed_to_mention_some_roles_in_thread: bool,
    /// This message will not trigger push and desktop notifications
    suppress_notifications: bool,
    /// This message is a voice message
    is_voice_message: bool,
}

impl Serialize for MessageFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bitfield = 0;

        if self.crossposted { bitfield |= 1 << 0; };
        if self.is_crosspost { bitfield |= 1 << 1; };
        if self.suppress_embeds { bitfield |= 1 << 2; };
        if self.source_message_deleted { bitfield |= 1 << 3; };
        if self.urgent { bitfield |= 1 << 4; };
        if self.has_thread { bitfield |= 1 << 5; };
        if self.ephemeral { bitfield |= 1 << 6; };
        if self.loading { bitfield |= 1 << 7; };
        if self.failed_to_mention_some_roles_in_thread { bitfield |= 1 << 8; };
        if self.suppress_notifications { bitfield |= 1 << 12; };
        if self.is_voice_message { bitfield |= 1 << 13; };

        serializer.serialize_u16(bitfield)
    }
}

macro_rules! bit_field {
    ($v:expr, $bit:expr) => {
        ($v >> $bit) & 1 != 0
    };
}

impl<'de> Deserialize<'de> for MessageFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u16::deserialize(deserializer) {
            Ok(v) => {
                Ok(Self {
                    crossposted: bit_field!(v, 0),
                    is_crosspost: bit_field!(v, 1),
                    suppress_embeds: bit_field!(v, 2),
                    source_message_deleted: bit_field!(v, 3),
                    urgent: bit_field!(v, 4),
                    has_thread: bit_field!(v, 5),
                    ephemeral: bit_field!(v, 6),
                    loading: bit_field!(v, 7),
                    failed_to_mention_some_roles_in_thread: bit_field!(v, 8),
                    suppress_notifications: bit_field!(v, 12),
                    is_voice_message: bit_field!(v, 13),
                })
            }
            Err(e) => { Err(e) }
        }
    }
}

