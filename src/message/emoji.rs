use serde::{Deserialize, Serialize};

use crate::id::DiscordID;
use crate::roles::Role;
use crate::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Emoji {
    /// Emoji id
    pub id: Option<DiscordID>,
    /// Emoji name
    pub name: Option<String>,
    /// Roles allowed to use this emoji
    pub roles: Option<Vec<Role>>,
    /// User that created this emoji
    pub user: Option<User>,
    /// Whether this emoji must be wrapped in colons
    pub require_colons: Option<bool>,
    /// Whether this emoji is managed
    pub managed: Option<bool>,
    /// Whether this emoji is animated
    pub animated: Option<bool>,
    /// Whether this emoji can be used. Maybe false due to loss of Server Boosts
    pub available: Option<bool>,
}

impl Emoji {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            roles: None,
            user: None,
            require_colons: None,
            managed: None,
            animated: None,
            available: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl Default for Emoji {
    fn default() -> Self {
        Self::new()
    }
}
