use anyhow::Result;
use streams_author::streams::ChannelAuthor;
use streams_author::http::api_server;

use std::sync::{Arc, Mutex};
use std::fs::File;
use rand::Rng;

#[tokio::main]
async fn main() -> Result<()> {
    let config: serde_json::Value = serde_json::from_reader(File::open("config.json").unwrap()).unwrap();
    let seed: String;
    if config["seed"].is_null() {
        let alph9 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
        seed = (0..10)
            .map(|_| alph9.chars().nth(rand::thread_rng().gen_range(0, 27)).unwrap())
            .collect::<String>();
    } else {
        seed = config["seed"].as_str().unwrap().to_string()
    }

    let mwm = config["mwm"].as_u64().unwrap() as u8;
    let node = config["node"].as_str().unwrap();
    let local_pow = config["local_pow"].as_bool().unwrap();
    let port = config["api_port"].as_u64().unwrap() as u16;

    println!("Making Streams channel...");
    println!("node = {}", config["node"]);
    println!("seed = {}", seed.as_str());
    let author = Arc::new(Mutex::new(ChannelAuthor::new(seed.as_str(), mwm, local_pow, node).unwrap()));
    let channel_address = author.lock().unwrap().get_announcement_id().unwrap();
    println!("\nChannel Address - {}:{}\n", channel_address.0, channel_address.1);

    match api_server::start(port, author).await {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!(e))
    }
}
