use serde::{Deserialize, Serialize};

use crate::id::DiscordID;
use crate::user::avatar_decoration::AvatarDecoration;
use crate::user::flags::UserFlags;
use crate::user::premium_type::PremiumType;

mod avatar_decoration;
mod flags;
mod premium_type;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// The user's id
    id: DiscordID,
    /// The user's username, not unique across the platform
    username: String,
    /// The user's Discord-tag
    discriminator: String,
    /// The user's display name, if it is set. For bots, this is the application name
    global_name: Option<String>,
    /// The user's avatar hash
    avatar: Option<String>,
    /// Whether the user belongs to an OAuth2 application
    bot: Option<bool>,
    /// Whether the user is an Official Discord System user (part of the urgent message system)
    system: Option<bool>,
    /// Whether the user has two-factor enabled on their account
    mfa_enabled: Option<bool>,
    /// The user's banner hash
    banner: Option<String>,
    /// The user's banner color encoded as an integer representation of hexadecimal color code
    accent_color: Option<u32>,
    /// The user's chosen language option
    locale: Option<String>,
    /// Whether the email on this account has been verified
    verified: Option<bool>,
    /// The user's email
    email: Option<String>,
    /// The flags on a user's account
    flags: Option<UserFlags>,
    /// The type of Nitro subscription on a user's account
    premium_type: Option<PremiumType>,
    /// The public flags on a user's account
    public_flags: Option<UserFlags>,
    /// Data for the user's avatar decoration
    avatar_decoration_data: Option<AvatarDecoration>,
}
