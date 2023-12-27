// Copyright (c) Tribufu. All Rights Reserved.

use actix_web::HttpRequest;
use tribufu_api::TribufuApi;

pub trait TribufuApiActixExtension {
    fn from_actix(req: &HttpRequest) -> Self;
    fn use_actix(&mut self, req: &HttpRequest);
}

impl TribufuApiActixExtension for TribufuApi {
    fn from_actix(req: &HttpRequest) -> Self {
        let mut api = Self::from_env();
        api.use_actix(req);
        api
    }

    fn use_actix(&mut self, req: &HttpRequest) {
        if let Some(authorization) = req.headers().get("Authorization") {
            let authorization = authorization.to_str().unwrap();

            if authorization.starts_with("ApiKey ") {
                self.use_api_key(authorization[7..].to_string());
            }

            if authorization.starts_with("Basic ") {
                self.use_basic(authorization[6..].to_string());
            }

            if authorization.starts_with("Bearer ") {
                self.use_bearer(authorization[7..].to_string());
            }
        }
    }
}
