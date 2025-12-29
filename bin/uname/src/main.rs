use coreutils::uname_main;

fn main() {
    std::process::exit(uname_main());
}
