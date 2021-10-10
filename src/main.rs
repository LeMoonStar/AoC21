use aoc21::{run, DayMode};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => run( match args[1].parse::<u8>() {
            Ok(n) => n,
            Err(_) => {eprintln!("Day arument must be a integer in range 1-25."); 0},
        }, DayMode::Both, &None),
        3 => run( match args[1].parse::<u8>() {
            Ok(n) => n,
            Err(_) => {eprintln!("Day arument must be a integer in range 1-25."); 0},
        }, match args[2].to_lowercase().as_str() {
            "1" | "one" | "first" => DayMode::One,
            "2" | "two" | "second" => DayMode::Two,
            "both" | "all" | "full" => DayMode::Both,
            _ => {eprintln!("(optional) Part argument must be one of the folowing: '1', 'one', 'first', '2', 'two', 'second', 'both', 'all', 'full'."); DayMode::Both}
        }, &None),
        0 => eprintln!("Usage: aoc21 day [part]"), // This normally shouldnt be called, it's just here so we dont panic even if this is for some reason the case.
        1 | _ => eprintln!("Usage: {} day [part]", args[0]),
    }
}
