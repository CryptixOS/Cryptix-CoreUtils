/*
 * Created by v1tr10l7 on 11.01.2026.
 * Copyright (c) 2024-2026, Szymon Zemke
 *
 * SPDX-License-Identifier: GPL-3
 */
use crate::syscall::*;
use std::ffi::CString;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

#[derive(Default)]
struct Options {
    recursive: bool,
    verbose: bool,
    changes: bool,
    quiet: bool,
}

fn print_help() {
    println!("Usage: chmod [OPTION]... MODE[,MODE] ... FILE...");
    println!("  or:  chmod [OPTION]... OCTAL-MODE FILE...");
    println!("  or:  chmod [OPTION]... --reference=RFILE FILE...");
    println!("Change the mode of each FILE to MODE.");
    println!("With --reference, change the mode of each file to that of RFILE.");
    println!();
    println!("  -c, --changes           like verbose but report only when a change is made");
    println!("  -f, --silent, --quiet   suppress most error messages");
    println!("  -v, --verbose           output a diagnostic for every file processed");
    println!("  -f, --silent, --quiet   suppress most error messages");
    println!("      --dereference       affect the referent of each symbolic link,");
    println!("                           rather than the symbolic link itself");
    println!("  -h, --no-dereference    affect each symbolic link, rather than the referent");
    println!("      --no-preserve-root  do not treat '/' specially (the default)");
    println!("      --preserve-root     fail to operate recursively on '/'");
    println!("      --reference=RFILE   use RFILE's mode instead of specifying MODE values.");
    println!("                           RFILE is always dereferenced if a symbolic link.");
    println!("  -R, --recursive         change files and directories recursively");
    println!();
    println!("The following options modify how a hierarchy is traversed when the -R");
    println!("option is also specified.  If more than one is specified, only the final");
    println!("one takes effect. -H is the default.");
    println!();
    println!("  -H                     if a command line argument is a symbolic link");
    println!("                         to a directory, traverse it");
    println!("  -L                     traverse every symbolic link to a directory");
    println!("                         encountered");
    println!("  -P                     do not traverse any symbolic links");
    println!();
    println!("     --help        display this help and exit");
    println!("     --version     output version information and exit");
    println!();
    println!("Each MODE is of the form '[ugoa]*([-+=]([rwxXst]*|[ugo]))+|[-+=][0-7]+'.");
}

fn print_version(prog: &str) {
    println!("{} 0.1", prog);
}

fn is_octal(s: &str) -> bool {
    s.chars().all(|c| ('0'..='7').contains(&c))
}

fn parse_symbolic(expr: &str, current: FileMode, umask: FileMode) -> Result<FileMode, String> {
    let mut mode = current;

    for clause in expr.split(',') {
        let mut chars = clause.chars().peekable();

        let mut who = 0;
        while let Some(&c) = chars.peek() {
            who |= match c {
                'u' => S_IRWXU,
                'g' => S_IRWXG,
                'o' => S_IRWXO,
                'a' => S_IRWXU | S_IRWXG | S_IRWXO,
                _ => break,
            };
            chars.next();
        }

        if who == 0 {
            who = (S_IRWXU | S_IRWXG | S_IRWXO) & !umask;
        }

        let op = chars.next().ok_or("missing operator")?;
        if !matches!(op, '+' | '-' | '=') {
            return Err("invalid operator".into());
        }

        let mut perm = 0;
        for c in chars {
            perm |= match c {
                'r' => S_IRUSR | S_IRGRP | S_IROTH,
                'w' => S_IWUSR | S_IWGRP | S_IWOTH,
                'x' => S_IXUSR | S_IXGRP | S_IXOTH,
                _ => return Err(format!("invalid permission '{}'", c)),
            };
        }

        perm &= who;

        match op {
            '+' => mode |= perm,
            '-' => mode &= !perm,
            '=' => {
                mode &= !who;
                mode |= perm;
            }
            _ => unreachable!(),
        }
    }

    Ok(mode)
}

fn compute_mode(mode_expr: &str, current: FileMode) -> Result<FileMode, String> {
    let umask = {
        let old = sys_umask(0);
        sys_umask(old);
        old
    };

    if is_octal(mode_expr) {
        Ok(u32::from_str_radix(mode_expr, 8).map_err(|_| "invalid octal mode")? as FileMode)
    } else {
        parse_symbolic(mode_expr, current, umask)
    }
}

fn chmod_file(path: &Path, mode_expr: &str, opts: &Options) -> Result<(), String> {
    let meta = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    let old = meta.permissions().mode();
    let new = compute_mode(mode_expr, old)?;

    if old != new {
        let cpath = CString::new(path.as_os_str().as_bytes()).unwrap();
        let rc = sys_chmod(cpath.as_ptr() as *const u8, new);
        if rc != 0 {
            return Err(std::io::Error::last_os_error().to_string());
        }

        if opts.verbose || opts.changes {
            println!(
                "mode of '{}' changed from {:o} to {:o}",
                path.display(),
                old & 0o777,
                new & 0o777
            );
        }
    } else if opts.verbose {
        println!("mode of '{}' retained as {:o}", path.display(), old & 0o777);
    }

    Ok(())
}

fn walk(path: &Path, mode: &str, opts: &Options) -> Result<(), String> {
    chmod_file(path, mode, opts)?;

    if opts.recursive && path.is_dir() {
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            walk(&entry.path(), mode, opts)?;
        }
    }

    Ok(())
}

pub fn chmod(args: &[String]) -> i32 {
    let mut opts = Options::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-c" | "--changes" => opts.changes = true,
            "-f" | "--silent" | "--quiet" => opts.quiet = true,
            "-v" | "--verbose" => opts.verbose = true,
            "-R" | "--recursive" => opts.recursive = true,
            "--help" => {
                print_help();
                return 0;
            }
            "--version" => {
                print_version(&args[0]);
                return 0;
            }
            _ if args[i].starts_with('-') => {
                if !opts.quiet {
                    eprintln!("{}: unknown option '{}'", args[0], args[i]);
                }
                return 1;
            }
            _ => break,
        }
        i += 1;
    }

    if i >= args.len() {
        print_help();
        return 1;
    }

    let mode = &args[i];
    i += 1;

    if i >= args.len() {
        print_help();
        return 1;
    }

    let mut failed = false;
    for f in &args[i..] {
        if let Err(e) = walk(Path::new(f), mode, &opts) {
            if !opts.quiet {
                eprintln!("{}: {}", f, e);
            }
            failed = true;
        }
    }

    if failed { 1 } else { 0 }
}
