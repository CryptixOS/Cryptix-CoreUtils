/*
 * Created by v1tr10l7 on 09.01.2026.
 * Copyright (c) 2024-2026, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
/// Print version
pub fn print_version(binary_name: &str) {
    println!(
        "{binary_name} (cryptix-coreutils) {0}\n",
        env!("CARGO_PKG_VERSION")
    );
}
