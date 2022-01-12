use serde::{ Deserialize, Serialize };

use super::{globals::{ExtractableGuild, ExtractableUser, Identifier, IntoMembers}, snowflake::Snowflake};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
    id: Snowflake,
    username: String,
    avatar: String,
    discriminator: String,
    public_flags: u64,
    flags: u64,
    banner: Option<String>,
    banner_color: Option<String>,
    accent_color: Option<u64>,
    bio: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    user: UserInfo,
    connected_accounts: Vec<Connection>,
    premium_since: Option<String>,
    premium_guild_since: Option<String>,
    mutual_guilds: Vec<MutualGuild>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Connection {
    #[serde(rename = "type")]
    #[serde(alias = "type")]
    _type: String,
    id: String,
    name: String,
    verified: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MutualGuild {
    id: Snowflake,
    nick: Option<String>
}


