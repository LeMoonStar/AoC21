// Nightly features
#![feature(int_abs_diff)]

use crate::days::Day;
use crate::days::DayImpl;
use aoc_macro::*;
use colored::*;
use lazy_static::lazy_static;
use mut_static::MutStatic;
use std::time::Duration;

mod days;

#[derive(Debug, Clone, PartialEq)]
pub enum Verbosity {
    None,
    Verbose,
    Developement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Part {
    One,
    Two,
    Both,
}

#[derive(Debug, Clone)]
pub struct Settings {
    verbosity: Verbosity,
}

impl Settings {
    fn set_verbosity(&mut self, new: Verbosity) {
        self.verbosity = new;
    }
}

lazy_static! {
    static ref VERBOSITY: MutStatic<Settings> = MutStatic::from(Settings {
        verbosity: Verbosity::None
    });
}

pub fn set_verbosity(new_verbosity: Verbosity) {
    let mut handle = VERBOSITY.write().unwrap();
    handle.set_verbosity(new_verbosity);
}

pub fn get_verbosity() -> Verbosity {
    VERBOSITY.read().unwrap().verbosity.clone()
}

#[macro_export]
macro_rules! vprintln {
    ($($arg:tt)*) => {
        if $crate::get_verbosity() ==  $crate::Verbosity::Verbose || $crate::get_verbosity() ==  $crate::Verbosity::Developement {
            println!(
                $($arg)*
            )
        }
    };
}

#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => {
        if $crate::get_verbosity() == $crate::Verbosity::Developement  {
            println!(
                $($arg)*
            )
        }
    };
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
    vprintln!("Test{}", "abc");

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
