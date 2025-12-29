pub fn yes(args: Vec<String>) -> i32 {
    let output = args
        .get(1..)
        .filter(|a| !a.is_empty())
        .map(|a| a.join(" "))
        .unwrap_or_else(|| "y".to_string());

    loop {
        println!("{}", output);
    }
}
