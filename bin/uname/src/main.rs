use coreutils::uname_main;
use std::process::exit;

fn main() {
    exit(uname_main());
}
