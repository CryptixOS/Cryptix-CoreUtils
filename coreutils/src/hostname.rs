/*
 * Created by v1tr10l7 on 09.01.2026.
 * Copyright (c) 2024-2026, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use crate::core::print_version;
use crate::syscall::{UtsName, sys_sethostname, sys_uname};

pub fn print_help() {
    println!(
        "
Usage: hostname [OPTION...] [NAME]
Show or set the system's host name.

  -a, --aliases              alias names
  -d, --domain               DNS domain name
  -f, --fqdn, --long         DNS host name or FQDN
  -F, --file=FILE            set host name or NIS domain name from FILE
  -i, --ip-addresses         addresses for the host name
  -s, --short                short host name
  -y, --yp, --nis            NIS/YP domain name
  -?, --help                 give this help list
      --usage                give a short usage message
  -V, --version              print program version

Mandatory or optional arguments to long options are also mandatory or optional
for any corresponding short options.
"
    );
}

pub fn hostname(args: &[String]) -> i32 {
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
            new_name => {
                let bytes = new_name.as_bytes();
                let ret = sys_sethostname(bytes.as_ptr(), bytes.len());
                if ret < 0 {
                    eprintln!("hostname: failed to set hostname\n");
                    return -1;
                }
                return 0;
            }
        }
    }

    let mut uts: UtsName = Default::default();

    let res = sys_uname(&mut uts);
    if res < 0 {
        eprintln!("uname syscall failed: {}", -res);
        return 1;
    }

    // nodename is the hostname
    let len = uts
        .nodename
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(uts.nodename.len());

    // safe slice up to the first null byte
    let name_bytes = &uts.nodename[..len];
    let hostname = std::str::from_utf8(name_bytes).unwrap_or("<invalid utf8>");

    println!("{}", hostname);
    0
}
