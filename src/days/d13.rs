use super::{Answer, Day, DayImpl};
use std::collections::HashSet;

const CURRENT_DAY: u8 = 13;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point(u16, u16);

#[derive(Debug, Clone, Copy)]
enum Instruction {
    FoldY(u16),
    FoldX(u16),
}

#[derive(Debug, Clone)]
pub struct Data(HashSet<Point>, Vec<Instruction>);

impl Data {
    fn new(s: &str) -> Self {
        let parts: Vec<&str> = s.split("\n\n").collect();
        let points: HashSet<Point> = parts[0]
            .lines()
            .map(|v| {
                let parts: Vec<u16> = v
                    .split(',')
                    .map(|v| v.parse::<u16>().expect("Failed while parsing"))
                    .collect();
                Point(parts[0], parts[1])
            })
            .collect();

        let instructions: Vec<Instruction> = parts[1]
            .lines()
            .map(|v| {
                // I was stuck here for ... 1.5 hours? cuz I only parsed the last character as number...
                if v.starts_with("fold along y=") {
                    Instruction::FoldY(
                        v.split_at(13)
                            .1
                            .parse::<u16>()
                            .expect("Failed while parsing."),
                    )
                } else {
                    Instruction::FoldX(
                        v.split_at(13)
                            .1
                            .parse::<u16>()
                            .expect("Failed while parsing."),
                    )
                }
            })
            .collect();

        Self(points, instructions)
    }

    fn execute_instruction(&mut self, inst_id: usize) {
        self.0 = self
            .0
            .iter()
            .map(|v| match self.1[inst_id] {
                Instruction::FoldY(l) => {
                    if v.1 > l {
                        Point(v.0, l - v.1.abs_diff(l))
                    } else {
                        *v
                    }
                }
                Instruction::FoldX(l) => {
                    if v.0 > l {
                        Point(l - v.0.abs_diff(l), v.1)
                    } else {
                        *v
                    }
                }
            })
            .collect();
    }

    fn run_all(&mut self) {
        for i in 0..self.1.len() {
            self.execute_instruction(i);
        }
    }

    fn create_map(&self) -> String {
        let mut points: HashSet<Point> = HashSet::with_capacity(self.0.len());

        let mut width: u16 = 0;
        let mut height: u16 = 0;

        for v in &self.0 {
            points.insert(*v);
            if v.0 > width {
                width = v.0;
            }
            if v.1 > height {
                height = v.1;
            }
        }

        let mut output: String = String::with_capacity((width * height + height) as usize);

        for y in 0..height + 1 {
            output += "\n";
            for x in 0..width + 1 {
                output += if points.contains(&Point(x, y)) {
                    "#"
                } else {
                    " "
                };
            }
        }

        output
    }
}

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test13.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (
            Answer::Number(17),
            Answer::String("\n#####\n#   #\n#   #\n#   #\n#####".to_owned()),
        )
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, Data::new(input))
    }

    fn one(&self, data: &mut Data) -> Answer {
        data.execute_instruction(0);
        Answer::Number(data.0.len() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        data.run_all();
        Answer::String(data.create_map())
    }
}
