// Copyright (c) Tribufu. All Rights Reserved

use dotenv::dotenv;
use std::env;
use tribufu::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let client_id = env::var("CLIENT_ID").unwrap().parse::<u64>().unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();

    let client = TribufuClient::new(client_id, client_secret).unwrap();
    let token = client.get_token().await.unwrap();

    println!("{:?}", token)
}
