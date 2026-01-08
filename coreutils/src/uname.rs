/*
 * Created by v1tr10l7 on 29.12.2025.
 * Copyright (c) 2024-2025, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use crate::syscall::{ID as SyscallID, syscall1};
use core::mem::MaybeUninit;

#[repr(C)]
pub struct UtsName {
    pub sysname: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
    pub domainname: [u8; 65],
}

pub fn uname() -> i32 {
    let mut uts = MaybeUninit::<UtsName>::uninit();

    let res = syscall1(SyscallID::UName, uts.as_mut_ptr() as usize);
    if res < 0 {
        eprintln!("uname syscall failed: {}", -res);
        return 1;
    }

    let uts = unsafe { uts.assume_init() };

    let sysname_bytes = &uts.sysname;
    let len = sysname_bytes
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(sysname_bytes.len());
    let sysname = str::from_utf8(&sysname_bytes[..len]).unwrap_or("Unknown");

    println!("{}", sysname);

    0
}
