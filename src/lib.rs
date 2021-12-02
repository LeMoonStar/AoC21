pub mod days;

#[cfg(test)]
pub mod tests;

use crate::days::run_day;
use crate::days::*;

pub enum DayMode {
    One,
    Two,
    Both,
}

enum Success<T> {
    Yes(T),
    No(&'static str),
}

pub fn run(day: u8, mode: &DayMode, input: &String) {
    let result: Success<Vec<Result<u64, Box<dyn std::error::Error>>>> = match day {
        1 => Success::Yes(run_day::<d01::Day1>(input, mode)),
        2 => Success::Yes(run_day::<d02::Day2>(input, mode)),
        0 | 26.. => Success::No("This is not a valid day. only days in range of 1-25 are valid."),
        _ => Success::No("This day is not implemented yet."),
    };

    match result {
        Success::Yes(results) => match mode {
            DayMode::One => match results[0] {
                Ok(r) => println!("The result for Part 1 is: {}", r),
                Err(_) => panic!("An error occured while processing Part 1."),
            },
            DayMode::Two => match results[0] {
                Ok(r) => println!("The result for Part 2 is: {}", r),
                Err(_) => panic!("An error occured while processing Part 2"),
            },
            DayMode::Both => {
                match results[0] {
                    Ok(r) => println!("The result for Part 1 is: {}", r),
                    Err(_) => panic!("An error occured while processing Part 1"),
                }
                match results[1] {
                    Ok(r) => println!("The result for Part 2 is: {}", r),
                    Err(_) => panic!("An error occured while processing Part 2"),
                }
            }
        },
        Success::No(e) => eprintln!("{}", e),
    }
}
