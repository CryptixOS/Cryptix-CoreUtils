/*
 * Created by v1tr10l7 on 01.01.2026.
 * Copyright (c) 2024-2026, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use crate::syscall::{ID as SyscallID, syscall0};

fn print_help() {
    println!(
        "Usage: whoami [OPTION]...
        Print the user name associated with the current effective user ID.
        Same as id -un.
        
        \t--help\tdisplay this help and exit
        \t--version\toutput version information and exit"
    );
}

pub fn whoami(args: Vec<String>) -> i32 {
    if args.iter().any(|a| a == "--help") {
        print_help();
        return 0;
    }
    if args.iter().any(|a| a == "--version") {
        //TODO(v1tr10l7): define public function printing the crate's version
        println!("Cryptix-CoreUtils 0.0.1");
        return 0;
    }

    let euid = syscall0(SyscallID::GetEUID);
    println!("Your Effective User ID => {euid}");

    return 0;
}
