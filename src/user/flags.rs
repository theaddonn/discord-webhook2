use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct UserFlags {
    /// Discord Employee
    staff: bool,
    /// Partnered Server Owner
    partner: bool,
    /// HypeSquad Events Member
    hypesquad: bool,
    /// Bug Hunter Level 1
    bug_hunter_level_1: bool,
    /// House Bravery Member
    hypesquad_online_house_1: bool,
    /// House Brilliance Member
    hypesquad_online_house_2: bool,
    /// House Balance Member
    hypesquad_online_house_3: bool,
    /// Early Nitro Supporter
    premium_early_supporter: bool,
    /// User is a team
    team_pseudo_user: bool,
    /// Bug Hunter Level 2
    bug_hunter_level_2: bool,
    /// Verified Bot
    verified_bot: bool,
    /// Early Verified Bot Developer
    verified_developer: bool,
    /// Moderator Programs Alumni
    certified_moderator: bool,
    /// Bot uses only HTTP interactions and is shown in the online member list
    bot_http_interactions: bool,
    /// User is an Active Developer
    active_developer: bool,
}

macro_rules! set_bit {
    ($v:expr, $bit:expr) => {
        $v |= 1 << $bit
    };
}

impl Serialize for UserFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bitfield = 0;
        
        if self.staff {
            set_bit!(bitfield, 0)
        }
        if self.partner {
            set_bit!(bitfield, 1)
        }
        if self.hypesquad {
            set_bit!(bitfield, 2)
        }
        if self.bug_hunter_level_1 {
            set_bit!(bitfield, 3)
        }
        if self.hypesquad_online_house_1 {
            set_bit!(bitfield, 6)
        }
        if self.hypesquad_online_house_2 {
            set_bit!(bitfield, 7)
        }
        if self.hypesquad_online_house_3 {
            set_bit!(bitfield, 8)
        }
        if self.premium_early_supporter {
            set_bit!(bitfield, 9)
        }
        if self.team_pseudo_user {
            set_bit!(bitfield, 10)
        }
        if self.bug_hunter_level_2 {
            set_bit!(bitfield, 14)
        }
        if self.verified_bot {
            set_bit!(bitfield, 16)
        }
        if self.verified_developer {
            set_bit!(bitfield, 17)
        }
        if self.certified_moderator {
            set_bit!(bitfield, 18)
        }
        if self.bot_http_interactions {
            set_bit!(bitfield, 19)
        }
        if self.active_developer {
            set_bit!(bitfield, 22)
        }
        
        serializer.serialize_u32(bitfield)
    }
}

macro_rules! get_bit {
    ($v:expr, $bit:expr) => {
        ($v >> $bit) & 1 != 0
    };
}

impl<'de> Deserialize<'de> for UserFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bitfield = u32::deserialize(deserializer)?;

        Ok(Self {
            staff: get_bit!(bitfield, 0),
            partner: get_bit!(bitfield, 1),
            hypesquad: get_bit!(bitfield, 2),
            bug_hunter_level_1: get_bit!(bitfield, 3),
            hypesquad_online_house_1: get_bit!(bitfield, 6),
            hypesquad_online_house_2: get_bit!(bitfield, 7),
            hypesquad_online_house_3: get_bit!(bitfield, 8),
            premium_early_supporter: get_bit!(bitfield, 9),
            team_pseudo_user: get_bit!(bitfield, 10),
            bug_hunter_level_2: get_bit!(bitfield, 14),
            verified_bot: get_bit!(bitfield, 16),
            verified_developer: get_bit!(bitfield, 17),
            certified_moderator: get_bit!(bitfield, 18),
            bot_http_interactions: get_bit!(bitfield, 19),
            active_developer: get_bit!(bitfield, 22),
        })
    }
}