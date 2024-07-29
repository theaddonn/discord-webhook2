mod tags;
mod flags;

use serde::{Deserialize, Serialize};
use crate::id::DiscordID;
use crate::roles::flags::RoleFlags;
use crate::roles::tags::RoleTags;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    /// Role id
    id: DiscordID,
    /// Role name
    name: String,
    /// Integer representation of hexadecimal color code
    color: u32,
    /// If this role is pinned in the user listing
    hoist: bool,
    /// Role icon hash
    icon_hash: Option<String>,
    /// Role unicode emoji
    unicode_emoji: Option<String>,
    /// Position of this role (roles with the same position are sorted by id)
    position: u32,
    /// Permission bit set
    permissions: String,
    /// Whether this role is managed by an integration
    managed: bool,
    /// Whether this role is mentionable
    mentionable: bool,
    tags: Option<RoleTags>,
    /// Role flags combined as a bitfield
    flags: RoleFlags,
}
