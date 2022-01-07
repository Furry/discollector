use std::num::ParseIntError;

use serde::{ Deserialize, Serialize };

use super::snowflake::Snowflake;

#[derive(Clone, Debug)]
pub struct EmittedGuild {
    pub id: Snowflake,
    pub name: String,
    pub invite: Option<String>
}

#[derive(Clone, Debug)]
pub enum Identifier {
    Icon,
    Snowflake
}

#[derive(Clone, Debug)]
pub struct EmittedUser {
    pub identifier: String,
    pub nickname: Option<String>,
    pub username: String,
    pub identifier_type: Identifier,
    pub guild: EmittedGuild
}

pub trait IntoMembers {
    fn members(&self) -> Vec<EmittedUser>;
}

pub trait ExtractableUser {
    fn user(&self) -> EmittedUser;
}

pub trait ExtractableGuild {
    fn guild(&self) -> EmittedGuild;
}

impl ExtractableGuild for EmittedUser {
    fn guild(&self) -> EmittedGuild {
        return self.guild.clone();
    }
}

impl ExtractableUser for EmittedUser {
    fn user(&self) -> EmittedUser {
        return self.clone();
    }
}