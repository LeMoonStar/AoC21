pub mod days;

use crate::days::*;

pub enum DayMode {
    One,
    Two,
    Both
}

pub fn run(day: u8, mode: DayMode, input: &Option<String>) {
    match day {
        1 =>  {let _ = d01::Day1::run(input.as_ref().unwrap_or(&("".to_string())), mode);},
        0 | 26.. => eprintln!("This is not a valid day. only days in range of 1-25 are valid."),
        _ => eprintln!("This day is not implemented yet."),
    }
}