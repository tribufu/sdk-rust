// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: UNLICENSED

use std::ffi::CString;
use std::os::raw::c_char;
use tribufu::TribufuApi;

#[no_mangle]
pub extern "C" fn tribufu_api_new() {}

#[no_mangle]
pub extern "C" fn tribufu_api_default() {}

#[no_mangle]
pub extern "C" fn tribufu_api_with_api_key() {}

#[no_mangle]
pub extern "C" fn tribufu_api_from_env() {}

#[no_mangle]
pub extern "C" fn tribufu_api_from_env_or_default() {}

#[no_mangle]
pub extern "C" fn tribufu_api_get_version() -> *const c_char {
    CString::new(TribufuApi::get_version()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn tribufu_api_get_user_agent() -> *const c_char {
    CString::new(TribufuApi::get_user_agent())
        .unwrap()
        .into_raw()
}
