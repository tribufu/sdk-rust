// Copyright (c) TribuFu. All Rights Reserved

pub fn InviteToTeam() {}

pub fn EnterTeam() {}

mod External {
    #[no_mangle]
    pub extern "C" fn InviteToTeam() {
        super::InviteToTeam();
    }

    #[no_mangle]
    pub extern "C" fn EnterTeam() {
        super::EnterTeam();
    }
}
