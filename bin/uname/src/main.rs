/*
 * Created by v1tr10l7 on 29.12.2025.
 * Copyright (c) 2024-2025, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use coreutils::uname;
use std::process::exit;

fn main() {
    exit(uname());
}
