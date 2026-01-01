/*
 * Created by v1tr10l7 on 01.01.2026.
 * Copyright (c) 2024-2026, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use std::env;
use std::io::{self, Write};

const VERSION: &str = "clear (cryptix-coreutils) 0.1";

fn print_help() {
    println!(
        "Usage: clear [OPTION]
Clear the terminal screen.

Options:
  -T TERM     use this terminal type
  -x          do not clear scrollback
  -V          output version information and exit
  --help      display this help and exit"
    );
}

pub fn clear_main() -> i32 {
    let mut args = env::args().skip(1);

    let mut term: Option<String> = None;
    let mut clear_scrollback = true;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-T" => {
                term = args.next();
                if term.is_none() {
                    eprintln!("clear: option '-T' requires an argument");
                    return 1;
                }
            }
            "-x" => {
                clear_scrollback = false;
            }
            "-V" => {
                println!("{}", VERSION);
                return 0;
            }
            "--help" => {
                print_help();
                return 0;
            }
            _ => {
                eprintln!("clear: unknown option '{}'", arg);
                return 1;
            }
        }
    }

    let _term = term
        .or_else(|| env::var("TERM").ok())
        .unwrap_or_else(|| "unknown".into());

    // Minimal ANSI-compatible clear
    let mut out = io::stdout();
    // Clear scrollback + screen
    if clear_scrollback {
        write!(out, "\x1b[3J\x1b[2J\x1b[H").unwrap();
    } else {
        write!(out, "\x1b[2J\x1b[H").unwrap();
    }

    out.flush().unwrap();
    return 0
}
