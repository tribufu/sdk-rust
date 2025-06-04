// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: UNLICENSED

use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn tribufu_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw(ptr);
    }
}
