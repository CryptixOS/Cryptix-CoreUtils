/*
 * Created by v1tr10l7 on 01.01.2026.
 * Copyright (c) 2024-2026, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use std::io::{self, Write};

pub fn echo(args: &[String]) -> i32 {
    let mut newline = true;
    let mut start = 1;

    if args.len() > 1 && args[1] == "-n" {
        newline = false;
        start = 2;
    }

    let mut out = io::stdout();

    let output = args[start..].join(" ");
    if write!(out, "{}", output).is_err() {
        return 1;
    }

    if newline && writeln!(out).is_err() {
        return 1;
    }

    0
}
