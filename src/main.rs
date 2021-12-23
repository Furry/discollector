pub mod mods;
pub mod discord;

use mods::context::Context;

use std::sync::{ Arc };

#[tokio::main]
async fn main() {
    let c = Arc::new(crate::mods::context::Context {});

    tokio::spawn(async move {
        c1.handle();
    });

    tokio::spawn(async move {
        c.clone().handle();
    });
}
