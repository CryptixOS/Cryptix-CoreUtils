/*
 * Created by v1tr10l7 on 09.01.2026.
 * Copyright (c) 2024-2026, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use crate::syscall::*;

const VERSION: &str = "cat (cryptix-coreutils) 0.1";

/// Print help message
fn print_help() {
    println!(
        "Usage: cat [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

Options:
  -n         number all output lines
  -E         display $ at end of each line
  --help     display this help and exit
  --version  output version information and exit
"
    );
}
/// Print version
fn print_version() {
    println!("{VERSION}\n");
}
const BUF_SIZE: usize = 4096;

/// Reads from fd and writes to stdout (fd = 1)
fn cat_fd(fd: usize, number_lines: bool, show_ends: bool, start_line: &mut usize) -> isize {
    let mut buf = [0u8; BUF_SIZE];

    loop {
        let n = sys_read(fd, buf.as_mut_ptr(), BUF_SIZE);
        if n < 0 {
            // read error
            return n;
        }
        if n == 0 {
            // EOF
            break;
        }

        let mut _written = 0;
        let mut i = 0;

        while i < n as usize {
            let line_start = i;

            while i < n as usize && buf[i] != b'\n' {
                i += 1;
            }

            let include_newline = i < n as usize && buf[i] == b'\n';
            let line_end = if include_newline { i + 1 } else { i };

            if number_lines {
                let line_no = *start_line;
                let s = format!("{line_no}\t");

                let _ = sys_write(1, s.as_ptr(), s.len());
                let _ = sys_write(1, b"\t".as_ptr(), 1);
                *start_line += 1;
            }

            let _ = sys_write(1, buf[line_start..line_end].as_ptr(), line_end - line_start);

            if show_ends && include_newline {
                let _ = sys_write(1, b"$\n".as_ptr(), 2);
            }

            i = line_end;
            _written = i;
        }
    }

    0
}

const O_RDONLY: isize = 0;
pub fn cat(args: &[String]) -> i32 {
    let mut number_lines = false;
    let mut show_ends = false;
    let mut file_start = 1;

    while file_start < args.len() {
        match args[file_start].as_str() {
            "--help" | "-h" => {
                print_help();
                return 0;
            }
            "--version" | "-v" => {
                print_version();
                return 0;
            }
            "-n" => {
                number_lines = true;
                file_start += 1;
            }
            "-E" => {
                show_ends = true;
                file_start += 1;
            }
            _ => break,
        }
    }

    let mut line_no = 1;
    if file_start >= args.len() {
        let _ = cat_fd(0, number_lines, show_ends, &mut line_no);
    } else {
        for i in file_start..args.len() {
            let path = &args[i];
            let fd = sys_open(path.as_ptr(), O_RDONLY, 0);
            if fd < 0 {
                println!("cat: cannot open file => `{path}`\n");
                continue;
            }

            let _ = cat_fd(fd as usize, number_lines, show_ends, &mut line_no);
            let _ = sys_close(fd as usize);
        }
    }

    0
}
