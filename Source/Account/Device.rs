// Copyright (c) TribuFu. All Rights Reserved

use libc::c_char;

pub fn GetDevice(id: *const c_char) {}

pub fn GetUserDevices(id: *const c_char) {}

mod External {
    use libc::c_char;

    #[no_mangle]
    pub extern "C" fn GetDevice(id: *const c_char) {
        super::GetDevice(id);
    }

    #[no_mangle]
    pub extern "C" fn GetUserDevices(id: *const c_char) {
        super::GetUserDevices(id);
    }
}
