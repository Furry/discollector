use std::{sync::{ Arc, RwLock, Mutex }, collections::VecDeque, ops::{Add, Sub}};
use futures::Future;
use reqwest::Request;
use tokio::time::{ Duration, sleep };

use super::structs::{snowflake::Snowflake, guild::Guild};

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
    pub async fn guild<I: Into<Snowflake>>(&self, id: I) {
        let url = format!("https://discord.com/api/v9/guilds/{}", id.into().to_string());

        let resp = self.client.get(url)
            .header("authorization", self.token.clone())
            .send()
            .await
            .unwrap();

        let status = resp.status();
        let text = &resp.text()
            .await
            .unwrap();

        if status == reqwest::StatusCode::OK {
            let guild: Guild = serde_json::from_str(text.as_str()).unwrap();
            println!("{}", serde_json::to_string_pretty(&guild).unwrap());
        }
    }
}