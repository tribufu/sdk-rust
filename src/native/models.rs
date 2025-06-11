// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use libc::c_void;

pub struct TribufuApiCallbackContext(pub(crate) *mut c_void);

impl TribufuApiCallbackContext {
    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.0
    }
}

unsafe impl Send for TribufuApiCallbackContext {}
unsafe impl Sync for TribufuApiCallbackContext {}

pub type TribufuApiGetUserInfoCallbackData = c_void;
pub type TribufuApiGetUserInfoCallback =
    extern "C" fn(*mut c_void, *const TribufuApiGetUserInfoCallbackData);
