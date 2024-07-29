use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum PollLayoutType {
    /// The, uhm, default layout type. XD
    Default = 1,
}
