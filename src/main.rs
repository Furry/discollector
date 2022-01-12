pub mod mods;
pub mod error;
pub mod discord;

#[macro_use]
pub mod macros;

use discord::{structs::widget::Widget, session::Session};
use reqwest;

use mods::context::Context;

use std::{sync::{ Arc, Mutex, RwLock }, collections::VecDeque};

use crate::discord::structs::globals::IntoMembers;


#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    let token = std::env::vars()
        .find(|k| k.0 == String::from("TESTING_TOKEN"))
        .unwrap().1;

    // let c = Arc::new(crate::mods::context::Context {});
    // let c1 = c.clone();

    // tokio::spawn(async move {
    //     let resp = reqwest::get("https://discord.com/api/guilds/244230771232079873/widget.json")
    //         .await
    //         .unwrap();

    //     let text = resp.text().await.unwrap();
    //     let x: Widget = serde_json::from_str(text.as_str()).unwrap();
    //     println!("{:?}", x);
    //     for member in x.members() {
    //         c1.handle(&member);
    //     }
    // }).await.unwrap();

    let ids = vec!["283058069305294848", "235804673578237952", "136815694095450113", "597337833027403778", "471889301622226946", "209002718897897472"];
    let session = Session::new(token);
    // let x = session.user("472474701919158275").await.unwrap();
    
    for id in ids {
        session.user(id).await.unwrap();
    }
    // session.guild("244230771232079873").await;

    // println!("Creating 1");
    // let f1 = async {};

    // println!("Creating 2");
    // let f2 = session.schedule(async {
    //     println!("Two!");
    // });

    // tokio::join!(f1);
    // tokio::spawn(async move {
    //     c.clone().handle();
    // });
}

