// Copyright (c) Tribufu. All Rights Reserved.

use crate::{TribufuApi, TribufuClient};

pub struct TribufuServer {
    server_id: u64,
    client: TribufuClient,
}

impl TribufuServer {
    pub fn new(server_id: u64, client_id: u64, client_secret: impl Into<String>) -> Self {
        Self {
            server_id,
            client: TribufuClient::new(client_id, client_secret),
        }
    }

    pub fn api(&self) -> &TribufuApi {
        self.client.api()
    }

    pub fn api_mut(&mut self) -> &mut TribufuApi {
        self.client.api_mut()
    }

    pub fn client(&self) -> &TribufuClient {
        &self.client
    }

    pub fn client_mut(&mut self) -> &mut TribufuClient {
        &mut self.client
    }

    pub fn server_id(&self) -> u64 {
        self.server_id
    }
}
