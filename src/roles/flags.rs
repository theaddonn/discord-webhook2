use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct RoleFlags {
    /// Members can select role in an onboarding prompt
    in_prompt: bool,
}

macro_rules! set_bit {
    ($v:expr, $bit:expr) => {
        $v |= 1 << $bit;
    };
}

impl Serialize for RoleFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bitfield = 0;

        if self.in_prompt {
            set_bit!(bitfield, 0);
        };

        serializer.serialize_u8(bitfield)
    }
}

macro_rules! get_bit {
    ($v:expr, $bit:expr) => {
        ($v >> $bit) & 1 != 0
    };
}

impl<'de> Deserialize<'de> for RoleFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bitfield = u8::deserialize(deserializer)?;

        Ok(Self {
            in_prompt: get_bit!(bitfield, 0),
        })
    }
}
