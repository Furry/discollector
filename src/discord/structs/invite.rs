use serde::{ Deserialize, Serialize };

use super::{globals::{ExtractableGuild, ExtractableUser, Identifier, IntoMembers}, snowflake::Snowflake};

#[derive(Deserialize, Serialize, Debug)]
pub struct InviteWelcomeChannel {
    #[serde(alias = "channel_id")]
    id: Snowflake,
    description: String,
    emoji_id: Option<String>,
    expires_at: Option<String>,
    guild: InviteGuild,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InviteChannel {
    id: Snowflake,
    name: String,
    #[serde(alias = "type")]
    _type: u8
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InviteGuild {
    id: Snowflake,
    name: String,
    splash: Option<String>,
    banner: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    features: Vec<String>,
    verification_level: u8,
    vanity_url_code: Option<String>,
    nsfw: bool,
    nsfw_level: u8,
    inviter: Inviter
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InviteWelcomeScreen {
    description: String,
    welcome_channels: Vec<InviteWelcomeChannel>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Inviter {
    id: Snowflake,
    username: String,
    avatar: String,
    discriminator: String,
    public_flags: u32,
    bot: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Invite {
    code: String,
    #[serde(alias = "type")]
    _type: u8,
    expires_at: Option<String>,
    guild: InviteGuild,
    channel: InviteChannel,
    inviter: Inviter,
    approximate_member_count: u32,
    approximate_presence_count: u32
}

impl ExtractableGuild for Invite {
    fn guild(&self) -> super::globals::EmittedGuild {
        super::globals::EmittedGuild {
            id: self.guild.id.clone(),
            name: self.guild.name.clone(),
            invite: Some(self.code.clone())
        }
    }
}

impl ExtractableUser for Invite {
    fn user(&self) -> super::globals::EmittedUser {
        super::globals::EmittedUser {
            identifier: self.inviter.id.clone().to_string(),
            nickname: None,
            username: self.inviter.username.clone(),
            identifier_type: Identifier::Snowflake,
            guild: self.guild(),
        }
    }
}