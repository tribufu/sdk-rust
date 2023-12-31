// Copyright (c) Tribufu. All Rights Reserved.

use actix_web::HttpRequest;

pub trait TribufuApiActixExtension {
    fn from_actix(req: &HttpRequest) -> Self;
}

/*
impl TribufuApiActixExtension for TribufuApi {
    fn from_actix(req: &HttpRequest) -> Self {
        let mut api = Self::with_client_from_env().unwrap_or_default();

        if let Some(authorization) = req.headers().get("Authorization") {
            let authorization = authorization.to_str().unwrap();

            if authorization.starts_with("Bearer ") {
                api = Self::with_user(authorization[7..].to_string());
            }
        }

        return api;
    }
}
*/
