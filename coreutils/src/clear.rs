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

pub fn clear_main(args: &[String]) -> i32 {
    let mut term: Option<String> = None;
    let mut clear_scrollback = true;

    let mut i = 1; // skip argv[0]

    while i < args.len() {
        match args[i].as_str() {
            "-T" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("clear: option '-T' requires an argument");
                    return 1;
                }
                term = Some(args[i].clone());
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
            arg => {
                eprintln!("clear: unknown option '{}'", arg);
                return 1;
            }
        }
        i += 1;
    }

    let _term = term
        .or_else(|| env::var("TERM").ok())
        .unwrap_or_else(|| "unknown".into());

    let mut out = io::stdout();

    if clear_scrollback {
        if write!(out, "\x1b[3J\x1b[2J\x1b[H").is_err() {
            return 1;
        }
    } else {
        if write!(out, "\x1b[2J\x1b[H").is_err() {
            return 1;
        }
    }

    if out.flush().is_err() {
        return 1;
    }

    0
}
