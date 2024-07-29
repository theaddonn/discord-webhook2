use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
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

macro_rules! set_bit {
    ($v:expr, $bit:expr) => {
        $v |= 1 << $bit;
    };
}

impl Serialize for MessageFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bitfield = 0;

        if self.crossposted {
            set_bit!(bitfield, 0);
        };
        if self.is_crosspost {
            set_bit!(bitfield, 1);
        };
        if self.suppress_embeds {
            set_bit!(bitfield, 2);
        };
        if self.source_message_deleted {
            set_bit!(bitfield, 3);
        };
        if self.urgent {
            set_bit!(bitfield, 4);
        };
        if self.has_thread {
            set_bit!(bitfield, 5);
        };
        if self.ephemeral {
            set_bit!(bitfield, 6);
        };
        if self.loading {
            set_bit!(bitfield, 7);
        };
        if self.failed_to_mention_some_roles_in_thread {
            set_bit!(bitfield, 8);
        };
        if self.suppress_notifications {
            set_bit!(bitfield, 12);
        };
        if self.is_voice_message {
            set_bit!(bitfield, 13);
        };

        serializer.serialize_u16(bitfield)
    }
}

macro_rules! get_bit {
    ($v:expr, $bit:expr) => {
        ($v >> $bit) & 1 != 0
    };
}

impl<'de> Deserialize<'de> for MessageFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bitfield = u16::deserialize(deserializer)?;

        Ok(Self {
            crossposted: get_bit!(bitfield, 0),
            is_crosspost: get_bit!(bitfield, 1),
            suppress_embeds: get_bit!(bitfield, 2),
            source_message_deleted: get_bit!(bitfield, 3),
            urgent: get_bit!(bitfield, 4),
            has_thread: get_bit!(bitfield, 5),
            ephemeral: get_bit!(bitfield, 6),
            loading: get_bit!(bitfield, 7),
            failed_to_mention_some_roles_in_thread: get_bit!(bitfield, 8),
            suppress_notifications: get_bit!(bitfield, 12),
            is_voice_message: get_bit!(bitfield, 13),
        })
    }
}
