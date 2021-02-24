// Copyright (c) TribuFu. All Rights Reserved

use libc::c_char;

pub fn GetMessage(id: *const c_char) {}

mod External {
    use libc::c_char;

    #[no_mangle]
    pub extern "C" fn GetMessage(id: *const c_char) {
        super::GetMessage(id);
    }
}
