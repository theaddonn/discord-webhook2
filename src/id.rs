use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DiscordID(pub u64);

impl Serialize for DiscordID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for DiscordID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;

        match u64::from_str(&str) {
            Ok(v) => { Ok(Self(v)) }
            Err(e) => { Err(Error::custom(format!("could not deserialize {str:?} into a MessageID: {e:?}"))) }
        }
    }
}
