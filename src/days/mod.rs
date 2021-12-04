use aoc_macro::mod_days;
use std::time::{Duration, Instant};

// Thanks to andi-makes with his AoC project https://github.com/andi-makes/aoc2021,
// this system is heavily inspired by his system.

pub struct Day<const DAY: u8>;

pub trait DayImpl<T> {
    /// Parses the test input.
    fn initTest() -> (Self, T)
    where
        Self: Sized;

    fn expected_results() -> (u64, u64);

    /// Parse input
    fn init(input: &String) -> (Self, T)
    where
        Self: Sized;

    /// Compute part 1
    fn one(&self, data: &mut T) -> u64;

    /// Compue part 2
    fn two(&self, data: &mut T) -> u64;

    /// Parse input and messure the time it took
    fn init_timed(input: &String) -> ((Self, T), Duration)
    where
        Self: Sized,
    {
        let s = Instant::now();
        (Self::init(input), s.elapsed())
    }

    /// Compute part 1 and messure the time it took
    fn one_timed(&self, data: &mut T) -> (u64, Duration) {
        let s = Instant::now();
        (self.one(data), s.elapsed())
    }

    /// Compute part 2 and messure the time it took
    fn two_timed(&self, data: &mut T) -> (u64, Duration) {
        let s = Instant::now();
        (self.two(data), s.elapsed())
    }

    /// Compute both parts
    fn run(input: &String) -> (u64, u64)
    where
        Self: Sized,
    {
        let (day, mut data) = Self::init(input);
        (day.one(&mut data), day.two(&mut data))
    }

    /// Compute both parts, and messure the time each step took
    fn run_timed(input: &String) -> (u64, u64, Duration, Duration, Duration)
    where
        Self: Sized,
    {
        let ((day, mut data), i_t) = Self::init_timed(input);
        let (one, one_t) = day.one_timed(&mut data);
        let (two, two_t) = day.two_timed(&mut data);

        (one, two, i_t, one_t, two_t)
    }
}

mod_days!();
