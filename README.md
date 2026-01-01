# Cryptix-CoreUtils

Cryptix-CoreUtils is an early-stage implementation of a minimal set of Unix-like core utilities written in Rust. The project is intended to provide basic userland tools for **CryptixOS** and to serve as a foundation for further expansion.

The focus is on correctness, clear structure, and tight integration with the operating system rather than feature completeness or full GNU compatibility.

[![Build](https://github.com/CryptixOS/Cryptix-CoreUtils/actions/workflows/build.yml/badge.svg)](https://github.com/CryptixOS/CryptixOS/actions/workflows/build.yml)
[![Check-Typos](https://github.com/CryptixOS/Cryptix-CoreUtils/actions/workflows/typos.yml/badge.svg)](https://github.com/CryptixOS/CryptixOS/actions/workflows/check-typos.yml)

---

## Goals

* Provide essential command-line utilities required for a usable userland
* Keep implementations simple and auditable
* Avoid unnecessary dependencies
* Allow direct interaction with system calls where appropriate
* Support incremental growth as CryptixOS evolves

This project is under active development and most of the utilities are incomplete.

---

## Implemented Utilities

Currently implemented or partially implemented utilities include:

* `clear`
* `ls`
* `uname`
* `yes`
* `true`
* `false`
* `echo`
* `pwd`

Behavior may differ from GNU coreutils and POSIX in some cases.

---

## Project Structure

The repository is organized as a Rust workspace:

```
.
├── Cargo.toml          # Workspace definition
├── coreutils/          # Shared library crate
│   └── src/
│       ├── lib.rs
│       ├── clear.rs
│       ├── ls.rs
│       ├── uname.rs
│       ├── yes.rs
│       ├── echo.rs
│       ├── pwd.rs
│       ├── true.rs
│       └── false.rs
├── bin/                # Thin binary crates
│   ├── clear/
│   ├── ls/
│   ├── uname/
│   ├── yes/
│   ├── echo/
│   ├── pwd/
│   ├── true/
│   └── false/
└── .github/workflows/  # CI configuration
```

Each binary crate contains only a `main.rs` that forwards execution to the corresponding function in the `coreutils` library.

---

## Design Principles

* **Library-first**: All logic lives in the `coreutils` library; binaries are thin wrappers.
* **Explicit exit codes**: Each utility returns an `i32` status code.
* **Minimal abstractions**: Prefer straightforward code over heavy frameworks.
* **System-oriented**: Utilities are written with OS development constraints in mind.

---

## Building

To build all utilities:

```sh
cargo build --workspace
```

To build a specific utility:

```sh
cargo build -p clear
```

---

## Testing

Basic tests (where available) can be run with:

```sh
cargo test --workspace
```

---

## Continuous Integration

The project uses GitHub Actions to:

* Build the full workspace
* Run tests
* Enforce formatting via `rustfmt`
* Run `clippy` with warnings treated as errors

---

## Licensing

This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)**.

SPDX identifier:

```
GPL-3.0
```

---

## Status

This project is at an early stage. Interfaces, behavior, and internal structure are subject to change without notice.

Contributions, experimentation, and refactoring are expected as development continues.

