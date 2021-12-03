use crate::days::Day;
use crate::days::DayImpl;
use aoc_macro::match_and_run_day;
use colored::*;
use std::time::Duration;

mod days;

pub enum Part {
    One,
    Two,
    Both,
}

pub fn run_day(day: u8, part: Part, input: &String) {
    println!("{} Day {}", "Starting".green(), day);
    println!("{}", "-----------------------".green().bold());
    let (one, two, init_t, one_t, two_t) = match_and_run_day!();

    println!("{}:", "Results".green().bold());
    println!("  {} took: {} µs", "Parsing".green(), init_t.as_micros());
    println!("  {}:", "Part 1".green());
    println!("      Solution: {}", one);
    println!("      Took:     {} µs", one_t.as_micros());
    println!("  {}:", "Part 2".green());
    println!("      Solution: {}", two);
    println!("      Took:     {} µs", two_t.as_micros());
}
