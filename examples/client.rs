// Copyright (c) Tribufu. All Rights Reserved

use tribufu::*;

#[tokio::main]
async fn main() {
    match TribufuClient::new(0, "client_secret") {
        Ok(client) => println!("client_id: {}", client.id()),
        Err(e) => println!("error: {:?}", e),
    }
}
