// Copyright (c) TribuFu. All Rights Reserved

use libc::c_char;

pub fn GetEmail(address: *const c_char) {}

pub fn GetUserEmails(id: *const c_char) {}

mod External {
    use libc::c_char;

    #[no_mangle]
    pub extern "C" fn GetEmail(address: *const c_char) {
        super::GetEmail(address);
    }

    #[no_mangle]
    pub extern "C" fn GetUserEmails(id: *const c_char) {
        super::GetUserEmails(id);
    }
}
