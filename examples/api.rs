// Copyright (c) Tribufu. All Rights Reserved

use tribufu::*;

#[tokio::main]
async fn main() {
    let api = TribufuApi::default();
    let games = api.get_games(Some(1)).await.unwrap();
    println!("{:?}", games);
}
