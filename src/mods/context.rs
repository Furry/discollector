use crate::discord::structs::{globals::{ ExtractableGuild, ExtractableUser }, invite::Invite};

pub struct Context {

}

impl Context {
    pub fn handle(&self, item: &(impl ExtractableUser + ExtractableGuild)) {
        let user = item.user();
        let guild = item.guild();

        println!("{}: {}", user.username, guild.name);
    }

    // pub fn handle_guild(&self, item: &impl ExtractableGuild) {

    // }
}

impl Context {
    pub async fn resolve_invite(&self, code: String) {
        let invite_url = format!("https://discord.com/api/v9/invites/{}?with_counts=true&with_expiration=true", code);

        let resp = reqwest::get(invite_url)
            .await
            .unwrap();

        let status = resp.status();
        let text = &resp.text()
            .await
            .unwrap();

        if status == reqwest::StatusCode::OK {
            let invite: Invite = serde_json::from_str(text.as_str()).unwrap();
            println!("{:?}", invite);
        }
    }
}