const HELP_MESSAGE: &str = "Usage: yes [STRING]...
  or:  yes OPTION
Repeatedly output a line with all specified STRING(s), or 'y'.

      -h , --help        display this help and exit
      -v    , --version     output version information and exit";

const VERSION_MESSAGE: &str = env!("CARGO_PKG_VERSION");

pub fn yes(args: Vec<String>) -> i32 {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{}", HELP_MESSAGE);

        return 0;
    }

    if args.iter().any(|a| a == "--version" || a == "-v") {
        println!("{}", VERSION_MESSAGE);

        return 0;
    }

    let output = args
        .get(1..)
        .filter(|a| !a.is_empty())
        .map(|a| a.join(" "))
        .unwrap_or_else(|| "y".to_string());

    loop {
        println!("{}", output);
    }
}
