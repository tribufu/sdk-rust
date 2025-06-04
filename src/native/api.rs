// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: UNLICENSED

use crate::models::{TribufuApiCallbackContext, TribufuApiGetUserInfoCallback};
use futures::lock::Mutex;
use libc::{c_char, c_void};
use once_cell::sync::Lazy;
use std::ffi::CString;
use std::ptr;
use tokio::runtime::Runtime;
use tribufu::apis::tribufu_generated_api::TribufuGeneratedApiClient;
use tribufu::TribufuApi;

static INSTANCE: Lazy<Mutex<Option<TribufuGeneratedApiClient>>> = Lazy::new(|| Mutex::new(None));
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

/// Gets the version of the Tribufu API.
#[no_mangle]
pub extern "C" fn tribufu_api_get_version() -> *const c_char {
    CString::new(TribufuApi::get_version()).unwrap().into_raw()
}

/// Gets the user agent string for the Tribufu API.
#[no_mangle]
pub extern "C" fn tribufu_api_get_user_agent() -> *const c_char {
    CString::new(TribufuApi::get_user_agent())
        .unwrap()
        .into_raw()
}

/// Initialize the Tribufu API instance.
///
/// This must be called before any other API functions.
#[no_mangle]
pub extern "C" fn tribufu_api_initialize() -> bool {
    let api = TribufuApi::from_env_or_default(None);

    if INSTANCE.try_lock().is_none() {
        return false;
    }

    let mut instance = INSTANCE.try_lock().unwrap();
    *instance = Some(api);

    true
}

/// Shutdown the Tribufu API instance.
///
/// This must be called when the API is no longer needed.
#[no_mangle]
pub extern "C" fn tribufu_api_shutdown() {
    if INSTANCE.try_lock().is_none() {
        return;
    }

    let mut instance = INSTANCE.try_lock().unwrap();
    *instance = None;
}

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn tribufu_api_get_user_info(
    context: *mut c_void,
    callback: TribufuApiGetUserInfoCallback,
) {
    let context = TribufuApiCallbackContext(context);

    RUNTIME.spawn(async move {
        let mut instance = INSTANCE.lock().await;
        if instance.is_none() {
            callback(context.as_ptr(), ptr::null());
            return;
        }

        let api = instance.as_mut();
        if api.is_none() {
            callback(context.as_ptr(), ptr::null());
            return;
        }

        let api = api.unwrap();

        // TODO: Implement logic here

        callback(context.as_ptr(), ptr::null());
    });
}
