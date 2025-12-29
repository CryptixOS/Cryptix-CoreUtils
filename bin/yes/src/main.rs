use coreutils::yes::yes;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    std::process::exit(yes(args));
}
