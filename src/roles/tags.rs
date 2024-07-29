use serde::{Deserialize, Serialize};

use crate::id::DiscordID;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleTags {
    /// The id of the bot this role belongs to
    bot_id: Option<DiscordID>,
    /// The id of the integration this role belongs to
    integration_id: Option<DiscordID>,
    /// Whether this is the guild's Booster role
    premium_subscriber: Option<()>,
    /// The id of this role's subscription sku and listing
    subscription_listing_id: Option<DiscordID>,
    /// Whether this role is available for purchase
    available_for_purchase: Option<()>,
    ///    Whether this role is a guild's linked role
    guild_connections: Option<()>,
}
