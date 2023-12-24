// Copyright (c) Tribufu. All Rights Reserved

use dotenv::dotenv;
use std::env;
use tribufu::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let client_id = env::var("CLIENT_ID").unwrap().parse::<u64>().unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();

    let mut client = TribufuClient::new(client_id, client_secret).unwrap();

    client.get_token(None).await.unwrap();

    let games = client.get_games().await.unwrap();

    games.iter().for_each(|game| {
        println!("{}", game.name);
    });
}
