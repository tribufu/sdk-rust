// Copyright (c) TribuFu. All Rights Reserved

use libc::c_char;

pub fn AddFriend(id1: *const c_char, id2: *const c_char) {}

pub fn AcceptFriend(id1: *const c_char, id2: *const c_char) {}

pub fn GetFriends(id: *const c_char) {}

mod External {
    use libc::c_char;

    #[no_mangle]
    pub extern "C" fn AddFriend(id1: *const c_char, id2: *const c_char) {
        super::AddFriend(id1, id2);
    }

    #[no_mangle]
    pub extern "C" fn AcceptFriend(id1: *const c_char, id2: *const c_char) {
        super::AcceptFriend(id1, id2);
    }

    #[no_mangle]
    pub extern "C" fn GetFriends(id: *const c_char) {
        super::GetFriends(id);
    }
}
