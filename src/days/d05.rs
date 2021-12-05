use super::{Day, DayImpl};
use crate::{dprintln, vprintln};
use std::collections::HashMap;

const CURRENT_DAY: u8 = 5;

#[derive(Debug, Clone)]
pub struct Line {
    start: (u16, u16),
    end: (u16, u16),
}

impl Line {
    fn new<S: AsRef<str>>(str: S) -> Self {
        let str = str.as_ref();
        let parsed = str
            .split(" -> ")
            .map(|v| {
                v.split(",")
                    .map(|v| v.parse::<u16>().expect("Expected a number in input."))
                    .collect::<Vec<u16>>()
            })
            .collect::<Vec<Vec<u16>>>();
        Self {
            start: (parsed[0][0], parsed[0][1]),
            end: (parsed[1][0], parsed[1][1]),
        }
    }

    fn is_diagonal(&self) -> bool {
        self.start.0 != self.end.0 && self.start.1 != self.end.1
    }

    fn get_non_diagonal_points(&self) -> Vec<(u16, u16)> {
        if self.is_diagonal() {
            vec![]
        } else {
            let mut points: Vec<(u16, u16)> = Vec::new();
            if self.start.0 != self.end.0 {
                for i in 0..self.start.0.abs_diff(self.end.0) + 1 {
                    points.push((self.start.0.min(self.end.0) + i, self.start.1));
                }
            } else {
                for i in 0..self.start.1.abs_diff(self.end.1) + 1 {
                    points.push((self.start.0, self.start.1.min(self.end.1) + i));
                }
            }

            points
        }
    }

    fn get_points(&self) -> Vec<(u16, u16)> {
        let mut points: Vec<(u16, u16)> = Vec::new();
        dprintln!("Calculationg lines of {:?}", self);

        let x_direction = if self.start.0 < self.end.0 {
            1
        } else {
            if self.start.0 == self.end.0 {
                0
            } else {
                -1
            }
        };
        let y_direction = if self.start.1 < self.end.1 {
            1
        } else {
            if self.start.1 == self.end.1 {
                0
            } else {
                -1
            }
        };

        dprintln!("Direction is: x={} y={}", x_direction, y_direction);

        for i in 0..self
            .start
            .0
            .abs_diff(self.end.0)
            .max(self.start.1.abs_diff(self.end.1))
            + 1
        {
            points.push((
                (self.start.0 as i32 + i as i32 * x_direction) as u16,
                (self.start.1 as i32 + i as i32 * y_direction) as u16,
            ))
        }

        points
    }
}

type Data = Vec<Line>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data)
    where
        Self: Sized,
    {
        Self::init(&include_str!("test_inputs/test05.txt").to_owned())
    }

    fn expected_results() -> (u64, u64) {
        (5, 12)
    }

    fn init(input: &String) -> (Self, Data)
    where
        Self: Sized,
    {
        (
            Self {},
            input
                .lines()
                .filter(|v| v.len() != 0)
                .map(|v| Line::new(v))
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> u64 {
        let mut map: HashMap<(u16, u16), u16> = HashMap::new();
        let mut overlap_count = 0;

        for l in data {
            for p in &l.get_non_diagonal_points() {
                map.insert(*p, map.get(&(p.0, p.1)).unwrap_or(&0) + 1);

                if *map.get(&(p.0, p.1)).unwrap_or(&0) == 2 {
                    overlap_count += 1;
                }
            }
        }

        overlap_count
    }

    fn two(&self, data: &mut Data) -> u64 {
        let mut map: HashMap<(u16, u16), u16> = HashMap::new();
        let mut overlap_count = 0;

        for l in data {
            for p in &l.get_points() {
                dprintln!(
                    "setting point: {:?}, which currently is: {}",
                    p,
                    map.get(&(p.0, p.1)).unwrap_or(&0)
                );

                if p.0 > 1000 {
                    vprintln!("dangerous: {:?} at {:?}", p, l)
                }
                if p.1 > 1000 {
                    vprintln!("dangerous: {:?} at {:?}", p, l)
                }

                map.insert(*p, map.get(&(p.0, p.1)).unwrap_or(&0) + 1);

                if *map.get(&(p.0, p.1)).unwrap_or(&0) == 2 {
                    dprintln!("Overlap found at {:?}!", p);
                    overlap_count += 1;
                }
            }
        }

        overlap_count
    }
}
