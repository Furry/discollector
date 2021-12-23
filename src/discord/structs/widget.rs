use serde::{ Deserialize, Serialize };

use super::globals::Snowflake;

#[derive(Deserialize, Serialize)]
pub struct Widget {
    id: Snowflake,
    name: String,
    instant_invite: Option<String>,
    channels: Vec<String>,
    members: Vec<WidgetMember>
}

#[derive(Deserialize, Serialize)]
pub struct WidgetMember {
    id: String,
    username: String,
    discriminatior: String,
    avatar: Option<String>,
    avatar_url: String,
    game: Option<Game>
}

#[derive(Deserialize, Serialize)]
pub struct Game {
    name: String
}