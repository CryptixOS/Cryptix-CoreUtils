/*
 * Created by v1tr10l7 on 09.01.2026.
 * Copyright (c) 2024-2026, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use crate::core::print_version;
use std::thread;
use std::time::Duration;

fn print_help() {
    println!(
        "
Usage: sleep NUMBER[SUFFIX]
  or:  sleep OPTION
Pause for NUMBER seconds, where NUMBER is an integer or floating-point.
SUFFIX may be 's','m','h', or 'd', for seconds, minutes, hours, days.
With multiple arguments, pause for the sum of their values.

  -h, --help                 display this help and exit
  -V, --version              print program version
"
    );
}
fn parse_duration(arg: &str) -> Option<Duration> {
    let (num_part, unit_part) = arg
        .trim()
        .chars()
        .partition::<String, _>(|c| c.is_digit(10) || *c == '.');

    if num_part.is_empty() {
        return None;
    }

    let value: f64 = match num_part.parse() {
        Ok(v) => v,
        Err(_) => return None,
    };

    let duration = match unit_part.as_str() {
        "" | "s" => Duration::from_secs_f64(value),
        "ms" => Duration::from_millis((value * 1.0) as u64),
        "m" => Duration::from_secs_f64(value * 60.0),
        "h" => Duration::from_secs_f64(value * 3600.0),
        _ => return None,
    };

    Some(duration)
}

pub fn sleep(args: &[String]) -> i32 {
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
            _ => {
                let duration = parse_duration(arg);
                match duration {
                    Some(dur) => thread::sleep(dur),
                    None => {
                        eprintln!("Error invalid duration '{}'", arg);
                        print_help();
                        return 1;
                    }
                }
            }
        }
    }

    0
}
