use crate::days::Day;
use crate::days::DayImpl;
use aoc_macro::*;
use colored::*;
use std::time::Duration;

mod days;

#[derive(Debug, Clone, PartialEq)]
pub enum Part {
    One,
    Two,
    Both,
}

pub fn run_day(day: u8, part: Part, input: &String) {
    println!("{} Day {}", "Starting".green().bold(), day);
    println!("{}", "-----------------------".green().bold());
    let (one, two, init_t, one_t, two_t) = match part {
        Part::Both => match_and_run_day_both!(),
        Part::One => {
            let (one, init_t, one_t) = match_and_run_day_one!();
            (one, 0, init_t, one_t, Duration::ZERO)
        }
        Part::Two => {
            let (two, init_t, two_t) = match_and_run_day_one!();
            (0, two, init_t, Duration::ZERO, two_t)
        }
    };

    println!("{}:", "Results".green().bold());
    println!("\t{}: {} µs", "Parsing time".green(), init_t.as_micros());
    if part == Part::Both || part == Part::One {
        println!("\t{}:", "Part 1".green());
        println!("\t\tSolution: {}", one);
        println!("\t\tTook:     {} µs", one_t.as_micros());
    }
    if part == Part::Both || part == Part::Two {
        println!("\t{}:", "Part 2".green());
        println!("\t\tSolution: {}", two);
        println!("\t\tTook:     {} µs", two_t.as_micros());
    }
}

pub fn test_day(day: u8, part: Part) {
    println!("{} Day {}", "Testing".green().bold(), day);
    println!("{}", "-----------------------".green().bold());
    match part {
        Part::Both => {
            let ((one_p, one_r, one_e), (two_p, two_r, two_e)) = match_and_test_day_both!();

            println!("{}:", "Results".green().bold());
            println!(
                "\t{}: {}",
                "Part 1".green(),
                match one_p {
                    true => {
                        "PASSED".green().bold()
                    }
                    false => {
                        "FAILED".red().bold()
                    }
                }
            );
            println!("\t\tResult:   {}", one_r);
            println!("\t\tExpected: {}", one_e);

            println!(
                "\t{}: {}",
                "Part 2".green(),
                match two_p {
                    true => {
                        "PASSED".green().bold()
                    }
                    false => {
                        "FAILED".red().bold()
                    }
                }
            );
            println!("\t\tResult:   {}", two_r);
            println!("\t\tExpected: {}", two_e);
        }
        Part::One => {
            let (one_p, one_r, one_e) = match_and_test_day_one!();

            println!("{}:", "Results".green().bold());
            println!(
                "\t{}: {}",
                "Part 1".green(),
                match one_p {
                    true => {
                        "PASSED".green().bold()
                    }
                    false => {
                        "FAILED".red().bold()
                    }
                }
            );
            println!("\t\tResult:   {}", one_r);
            println!("\t\tExpected: {}", one_e);
        }
        Part::Two => {
            let (two_p, two_r, two_e) = match_and_test_day_two!();

            println!("{}:", "Results".green().bold());
            println!(
                "\t{}: {}",
                "Part 2".green(),
                match two_p {
                    true => {
                        "PASSED".green().bold()
                    }
                    false => {
                        "FAILED".red().bold()
                    }
                }
            );
            println!("\t\tResult:   {}", two_r);
            println!("\t\tExpected: {}", two_e);
        }
    }
}
