// Copyright (c) TribuFu. All Rights Reserved

use libc::c_char;

pub fn Login(name: *const c_char, password: *const c_char) {}

pub fn Logout() {}

pub fn Refresh() {}

pub fn Register(name: *const c_char, email: *const c_char, password: *const c_char) {}

mod External {
    use libc::c_char;

    #[no_mangle]
    pub extern "C" fn Login(name: *const c_char, password: *const c_char) {
        super::Login(name, password);
    }

    #[no_mangle]
    pub extern "C" fn Logout() {
        super::Logout();
    }

    #[no_mangle]
    pub extern "C" fn Refresh() {
        super::Refresh();
    }

    #[no_mangle]
    pub extern "C" fn Register(name: *const c_char, email: *const c_char, password: *const c_char) {
        super::Register(name, email, password);
    }
}
