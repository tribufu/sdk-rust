// Copyright (c) Tribufu. All Rights Reserved

use tribufu::*;

#[tokio::main]
async fn main() {
    let api = TribufuApi::with_client_from_env().unwrap_or_default();
    let games = api.get_games(Some(1)).await.unwrap();
    println!("{:?}", games);
}
