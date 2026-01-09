/*
 * Created by v1tr10l7 on 09.01.2026.
 * Copyright (c) 2024-2026, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use crate::core::print_version;

fn print_help() {
    println!(
        "
Usage: id [OPTION...] [USER]
Print user and group information for each specified USER,
or (when USER omitted) for the current process.

  -a                         ignore, for compatibility with other versions
  -Z, --context              print only the security context of the process
  -g, --group                print only the effective group ID
  -G, --groups               print all group IDs
  -n, --name                 print a name instead of a number, for -u,-g,-G
  -r, --real                 print the real ID instead of the effective ID, with -u,-g-G
  -u, --user                 print only the effective user ID
  -z, --zero                 delimit entries with NUL characters, not whitespace;
                               not permitted in default format
  -h, --help                 display this help and exit
  -V, --version              print program version

Without any OPTION, print some useful set of identified information.
"
    );
}

pub fn id(args: &[String]) -> i32 {
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                return 0;
            }
            "--version" | "-v" => {
                print_version(args[0].as_str());
                return 0;
            }
            _ => {}
        }
    }

    0
}
