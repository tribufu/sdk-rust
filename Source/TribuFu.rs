// Copyright (c) TribuFu. All Rights Reserved

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

pub mod Account;
pub mod Friends;
pub mod Matchmaking;
pub mod Messages;

#[no_mangle]
pub extern "C" fn Hello() -> i32 {
    return 1;
}
