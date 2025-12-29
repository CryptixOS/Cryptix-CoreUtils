use coreutils::ls::ls;

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    exit(ls(args));
}
