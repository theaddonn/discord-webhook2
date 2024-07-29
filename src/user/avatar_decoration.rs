use serde::{Deserialize, Serialize};
use crate::id::DiscordID;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvatarDecoration {
    /// The avatar decoration hash
    asset: String,
    /// ID of the avatar decoration's SKU
    sku_id: DiscordID,
}
