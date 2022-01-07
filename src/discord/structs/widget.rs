use serde::{ Deserialize, Serialize };

use super::{globals::{ExtractableGuild, ExtractableUser, Identifier, IntoMembers}, snowflake::Snowflake};

#[derive(Deserialize, Serialize, Debug)]
pub struct Widget {
    id: Snowflake,
    name: String,
    instant_invite: Option<String>,
    channels: Vec<WidgetChannel>,
    members: Vec<WidgetMember>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WidgetMember {
    id: String,
    username: String,
    discriminatior: Option<String>,
    avatar: Option<String>,
    avatar_url: String,
    game: Option<WidgetGame>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WidgetGame {
    name: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WidgetChannel {
    id: Snowflake,
    name: String,
    position: u16
}

impl IntoMembers for Widget {
    fn members(&self) -> Vec<super::globals::EmittedUser> {
        let mut body: Vec<super::globals::EmittedUser> = Vec::new();
        let guild = super::globals::EmittedGuild {
            id: self.id.clone(),
            name: self.name.clone(),
            invite: self.instant_invite.clone()
        };

        for member in &self.members {
            body.push(super::globals::EmittedUser {
                identifier: member.avatar_url.clone(),
                nickname: None.clone(),
                username: member.username.clone(),
                identifier_type: Identifier::Icon.clone(),
                guild: guild.clone(),
            });
        }

        return body;
    }
}

// fn guild(&self) -> super::globals::EmittedGuild {
//     return super::globals::EmittedGuild {
//         id: self.id,
//         name: self.name,
//         invite: self.instant_invite
//     };
// }

// fn user(&self) -> super::globals::EmittedUser {
//     return super::globals::EmittedUser {
//         identifier: self.avatar_url,
//         nickname: None,
//         username: self.name,
//         identifier_type: Identifier::Icon,
//         guild: todo!(),
//     }
// }