// Copyright (c) Tribufu. All Rights Reserved.

/*
use actix_web::HttpRequest;

pub trait TribufuApiActixExtension {
    fn use_anonymous(req: &HttpRequest) -> Self;
}

impl TribufuApi {
    pub fn from_actix(req: &HttpRequest) -> Self {
        let mut api = Self::default();

        if let Some(api_key) = req.headers().get("X-Tribufu-Api-Key") {
            api.use_api_key(api_key.to_str().unwrap().to_string());
        }

        if let Some(authorization) = req.headers().get("Authorization") {
            let authorization = authorization.to_str().unwrap();

            if authorization.starts_with("Bearer ") {
                api.use_token(authorization[7..].to_string());
            }
        }

        api
    }
}
*/
