use super::Day;

#[derive(Debug, Copy, Clone)]
enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

pub struct Day2 {
    input: Vec<Command>,
}

impl Day for Day2 {
    fn new(input: &String) -> Self {
        Day2 {
            input: input
                .lines()
                .filter(|v| v.len() != 0)
                .map(|v| match v.len() {
                    4 => Command::Up(v.get(v.len() - 1..).unwrap().parse::<u64>().unwrap()),
                    6 => Command::Down(v.get(v.len() - 1..).unwrap().parse::<u64>().unwrap()),
                    9 => Command::Forward(v.get(v.len() - 1..).unwrap().parse::<u64>().unwrap()),
                    _ => panic!("malformed input."),
                })
                .collect(),
        }
    }

    fn first(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut depth: u64 = 0;
        let mut horizontal_pos: u64 = 0;
        for c in &self.input {
            match c {
                Command::Down(v) => depth = depth + (*v as u64),
                Command::Up(v) => depth = depth - (*v as u64),
                Command::Forward(v) => horizontal_pos = horizontal_pos + (*v as u64),
            }
        }
        Ok((depth * horizontal_pos) as u64)
    }

    fn second(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut depth: u64 = 0;
        let mut aim: u64 = 0;
        let mut horizontal_pos: u64 = 0;
        for c in &self.input {
            match c {
                Command::Down(v) => aim = aim + v,
                Command::Up(v) => aim = aim - v,
                Command::Forward(v) => {
                    horizontal_pos = horizontal_pos + v;
                    depth = depth + v * aim;
                }
            }
        }
        Ok(depth * horizontal_pos)
    }
}
