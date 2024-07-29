use serde::{Deserialize, Serialize};
use crate::id::DiscordID;
use crate::roles::Role;
use crate::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Emoji {
    /// Emoji id
    id: Option<DiscordID>,
    /// Emoji name
    name: Option<String>,
    /// Roles allowed to use this emoji
    roles: Option<Vec<Role>>,
    /// User that created this emoji
    user: Option<User>,
    /// Whether this emoji must be wrapped in colons
    require_colons: Option<bool>,
    /// Whether this emoji is managed
    managed: Option<bool>,
    /// Whether this emoji is animated
    animated: Option<bool>,
    /// Whether this emoji can be used. Maybe false due to loss of Server Boosts
    available: Option<bool>
}
