// Copyright (c) TribuFu. All Rights Reserved

use libc::c_char;

pub fn GetRole(id: *const c_char) {}

pub fn GetUserRoles(id: *const c_char) {}

mod External {
    use libc::c_char;

    #[no_mangle]
    pub extern "C" fn GetRole(id: *const c_char) {
        super::GetRole(id);
    }

    #[no_mangle]
    pub extern "C" fn GetUserRoles(id: *const c_char) {
        super::GetUserRoles(id);
    }
}
