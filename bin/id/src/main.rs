/*
 * Created by v1tr10l7 on 01.01.2026.
 * Copyright (c) 2024-2026, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use coreutils::id;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    exit(id(&args));
}
