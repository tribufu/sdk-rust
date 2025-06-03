// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use dotenv::dotenv;
use tribufu::apis::tribufu_generated_api::TribufuGeneratedApi;
use tribufu::TribufuApi;

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    let tribufu = TribufuApi::from_env_or_default(None);
    let user_info = tribufu.get_user_info().await.unwrap();
    println!("{:?}", user_info);
}
