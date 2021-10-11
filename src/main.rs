use std::fs::read_to_string;
use aoc21::{run, DayMode};

fn get_input(day: u8) -> String {
    eprintln!("test");
    match read_to_string(format!("input{:02}.txt", day)) {
        Ok(input) => input,
        Err(_) => {
            let mut input = String::new();
            let stdin = std::io::stdin();

            println!("Input file 'input{:02}.txt' not detected, please paste the challange input, and then press {}", day, match cfg!(windows) { true => "CTRL-Z", _ => "CTRL-D"});

            loop {
                match stdin.read_line(&mut input) {
                    Ok(l) => if l == 0 {break;},
                    Err(err) => panic!("Error encountered while trying to read stdin: {}", err),
                }
            }

            println!("\nInput ended.");

            input
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => match args[1].parse::<u8>() {
                Ok(n) => run(n, &DayMode::Both, &get_input(n)),
                Err(_) => eprintln!("Day arument must be a integer in range 1-25."),
            },
        3 => match args[1].parse::<u8>() {
                Ok(n) => run(n, match args[2].to_lowercase().as_str() {
                    "1" | "one" | "first" => &DayMode::One,
                    "2" | "two" | "second" => &DayMode::Two,
                    "both" | "all" | "full" => &DayMode::Both,
                    _ => {eprintln!("(optional) Part argument must be one of the folowing: '1', 'one', 'first', '2', 'two', 'second', 'both', 'all', 'full'."); &DayMode::Both}
                }, &get_input(n)),
                Err(_) => eprintln!("Day arument must be a integer in range 1-25."),
            },
        0 => eprintln!("Usage: aoc21 day [part]"), // This normally shouldnt be called, it's just here so we dont panic even if this is for some reason the case.
        1 | _ => eprintln!("Usage: {} day [part]", args[0]),
    }
}
