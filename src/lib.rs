pub mod days;

use crate::days::*;

pub enum DayMode {
    One,
    Two,
    Both,
}

pub fn run(day: u8, mode: &DayMode, input: &String) {
    match day {
        1 => {
            let results = d01::Day1::run(input, mode);
            match mode {
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
            }
        }
        0 | 26.. => eprintln!("This is not a valid day. only days in range of 1-25 are valid."),
        _ => eprintln!("This day is not implemented yet."),
    }
}
