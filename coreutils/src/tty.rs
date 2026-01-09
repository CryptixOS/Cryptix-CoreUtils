/*
 * Created by v1tr10l7 on 09.01.2026.
 * Copyright (c) 2024-2026, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use crate::core::print_version;
use crate::syscall::*;
use std::ffi::CStr;

fn print_help() {
    println!("Usage: tty [OPTION]");
    println!("Print the file name of the terminal connected to standard input.");
    println!();
    println!("  -s, --silent, --quiet   print nothing, only return status");
    println!("  -h, --help              display this help and exit");
    println!("  -v, --version           output version information and exit");
}

pub fn tty(args: &[String]) -> i32 {
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-s" | "--silent" | "--quiet" => return 0,
            "-h" | "--help" => {
                print_help();
                return 0;
            }
            "-v" | "--version" => {
                print_version(&args[0]);
                return 0;
            }
            _ => {
                eprintln!("{}: unknown option '{}'", args[0], arg);
                return 1;
            }
        }
    }

    let pid = sys_getpid();
    let path = format!("/proc/{}/fd/0\0", pid);

    let mut buf: [u8; 256] = [0; 256];
    let len = sys_readlink(path.as_ptr(), buf.as_mut_ptr(), buf.len() - 1);

    if len < 0 {
        return 1;
    }

    buf[len as usize] = 0;
    let tty = unsafe { CStr::from_ptr(buf.as_ptr() as *const i8) }.to_string_lossy();

    println!("{}", tty);
    0
}
