// Copyright (c) Tribufu. All Rights Reserved

use tribufu::*;

#[tokio::main]
async fn main() {
    let mut api = TribufuApi::default();
    api.use_anonymous();
}
