use serde::{ Deserialize, Serialize };

pub type Snowflake = String;

pub struct EmittedGuild {
    id: Snowflake,
    name: String,
    invite: Option<String>
}

pub enum Identifier {
    Icon,
    Snowflake
}

pub struct EmittedUser {
    identifier: String,
    nickname: Option<String>,
    username: String,
    identifier_type: Identifier,
    guild: EmittedGuild
}

pub trait IntoMembers {
    fn members() -> Vec<EmittedUser>;
}