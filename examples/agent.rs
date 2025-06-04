// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use tribufu::TribufuApi;

#[tokio::main]
async fn main() {
    let user_agent = TribufuApi::get_user_agent();
    println!("{}", user_agent);
}
