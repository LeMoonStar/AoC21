use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 2;

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

pub type Data = Vec<Command>;

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test02.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(150), Answer::Number(900))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| match v.len() {
                    4 => Command::Up(v.get(v.len() - 1..).unwrap().parse::<u64>().unwrap()),
                    6 => Command::Down(v.get(v.len() - 1..).unwrap().parse::<u64>().unwrap()),
                    9 => Command::Forward(v.get(v.len() - 1..).unwrap().parse::<u64>().unwrap()),
                    _ => panic!("malformed input."),
                })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut depth: u64 = 0;
        let mut horizontal_pos: u64 = 0;
        for c in data {
            match c {
                Command::Down(v) => depth = depth + (*v as u64),
                Command::Up(v) => depth = depth - (*v as u64),
                Command::Forward(v) => horizontal_pos = horizontal_pos + (*v as u64),
            }
        }
        Answer::Number((depth * horizontal_pos) as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut depth: u64 = 0;
        let mut aim: u64 = 0;
        let mut horizontal_pos: u64 = 0;
        for c in data {
            match c {
                Command::Down(v) => aim = aim + *v,
                Command::Up(v) => aim = aim - *v,
                Command::Forward(v) => {
                    horizontal_pos = horizontal_pos + *v;
                    depth = depth + *v * aim;
                }
            }
        }
        Answer::Number(depth * horizontal_pos)
    }
}
