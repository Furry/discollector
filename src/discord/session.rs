use std::{sync::{ Arc, RwLock, Mutex }, collections::VecDeque, ops::{Add, Sub}};
use futures::Future;
use reqwest::Request;
use tokio::time::{ Duration, sleep };

use crate::error::DiscollectorError;

use super::structs::{snowflake::Snowflake, guild::Guild, user::User};

use anyhow::{ Result, anyhow };

pub struct Session {
    running: bool,
    token: String,
    client: reqwest::Client
}

impl Session {
    pub fn new<S: Into<String>>(token: S) -> Self {
        Self {
            running: false,
            token: token.into(),
            client: reqwest::Client::new()
        }
    }
}

impl Session {
    pub async fn guild<I: Into<Snowflake>>(&self, id: I) -> Result<Guild> {
        let url = format!("https://discord.com/api/v9/guilds/{}", id.into().to_string());

        let resp = self.client.get(url)
            .header("authorization", self.token.clone())
            .send()
            .await?;

        let status = resp.status();
        let text = &resp.text().await?;

        if status == reqwest::StatusCode::OK {
            return Ok(serde_json::from_str(text.as_str())?);
        } else {
            return Err(anyhow!("Unknown Error"));
        }
    }

    pub async fn user_with_guild<I: Into<Snowflake>>(&self, id: I, guild_id: I) {

    }

    pub async fn user<I: Into<Snowflake>>(&self, id: I) -> Result<User> {
        let url = format!("https://discord.com/api/v9/users/{}/profile", id.into().to_string());

        let resp = self.client.get(url)
            .header("authorization", self.token.clone())
            .send()
            .await?;

        let status = resp.status();
        let text = &resp.text().await?;
        
        if status == reqwest::StatusCode::OK {
            return Ok(serde_json::from_str(text.as_str()).unwrap());
        } else {
            return Err(anyhow!("Unknown Error"));
        }
    }
}