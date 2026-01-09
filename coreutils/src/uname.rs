/*
 * Created by v1tr10l7 on 29.12.2025.
 * Copyright (c) 2024-2025, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use crate::syscall::{UtsName, sys_uname};

pub fn uname() -> i32 {
    let mut uts: UtsName = UtsName::default();

    let res = sys_uname(&mut uts);
    if res < 0 {
        eprintln!("uname syscall failed: {}", -res);
        return 1;
    }

    let sysname_bytes = &uts.sysname;
    let len = sysname_bytes
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(sysname_bytes.len());
    let sysname = str::from_utf8(&sysname_bytes[..len]).unwrap_or("Unknown");

    println!("{}", sysname);

    0
}
