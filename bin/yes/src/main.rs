use coreutils::yes::yes;

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    exit(yes(args));
}
