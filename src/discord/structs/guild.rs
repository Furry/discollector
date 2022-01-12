use serde::{ Deserialize, Serialize };

use super::{globals::{ExtractableGuild, ExtractableUser, Identifier, IntoMembers}, snowflake::Snowflake};

#[derive(Deserialize, Serialize, Debug)]
pub struct Guild {
    id: Snowflake,
    name: String,
    icon: String,
    description: String,
    splash: String,
    discovery_splash: String,
    features: Vec<String>,
    emojis: Vec<Emoji>,
    stickers: Vec<Sticker>,
    banner: String,
    owner_id: Snowflake,
    application_id: Option<Snowflake>,
    region: String,
    afk_channel_id: Option<Snowflake>,
    afk_timeout: Option<i32>,
    system_channel_id: Snowflake,
    widget_enabled: bool,
    widget_channel_id: Option<Snowflake>,
    verification_level: i32,
    roles: Vec<Role>,
    default_message_notifications: i32,
    mfa_level: i32,
    explicit_content_filter: i32,
    max_presences: Option<i32>,
    max_members: i32,
    max_video_channel_users: i32,
    vanity_url_code: String,
    premium_tier: i32,
    premium_subscription_count: i32,
    system_channel_flags: i32,
    preferred_locale: String,
    rules_channel_id: String,
    hub_type: Option<String>,
    premium_progress_bar_enabled: bool,
    nsfw: bool,
    nsfw_level: i32
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Emoji {
    name: String,
    roles: Vec<Snowflake>,
    id: Snowflake,
    require_colons: bool,
    managed: bool,
    animated: bool,
    available: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sticker {
    id: Snowflake,
    name: String,
    tags: String,
    #[serde(rename = "type")]
    #[serde(alias = "type")]
    _type: i32,
    format_type: i32,
    description: Option<String>,
    asset: String,
    available: bool,
    guild_id: Snowflake
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tags {
    bot_id: Option<Snowflake>,
    premium_subscriber: Option<bool>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Role {
    id: Snowflake,
    name: String,
    permissions: String,
    position: i32,
    color: i32,
    hoist: bool,
    managed: bool,
    mentionable: bool,
    icon: Option<String>,
    unicode_emoji: Option<String>,
    tags: Option<Tags>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GuildMember {
    roles: Vec<Snowflake>,
    nick: Option<String>,
    avatar: Option<String>,
    premium_since: Option<String>,
    joined_at: String,
    is_pending: bool,
    communication_disabled_until: Option<String>,
    bio: String,
    banner: Option<String>,
    mute: bool,
    deaf: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GuildUser {
    id: Snowflake,
    username: String,
    avatar: Option<String>,
    discriminator: String,
    public_flags: u64
}