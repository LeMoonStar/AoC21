use super::{Day, DayImpl};

const CURRENT_DAY: u8 = 10;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BracketType {
    Round,
    Square,
    Curly,
    Angle,
}

impl BracketType {
    fn get_corruption_value(&self) -> u64 {
        match self {
            BracketType::Round => 3,
            BracketType::Square => 57,
            BracketType::Curly => 1197,
            BracketType::Angle => 25137,
        }
    }

    fn get_incomplete_value(&self) -> u64 {
        match self {
            BracketType::Round => 1,
            BracketType::Square => 2,
            BracketType::Curly => 3,
            BracketType::Angle => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bracket {
    Opening(BracketType),
    Closing(BracketType),
}

impl Bracket {
    fn new(c: char) -> Self {
        match c {
            '(' => Self::Opening(BracketType::Round),
            '[' => Self::Opening(BracketType::Square),
            '{' => Self::Opening(BracketType::Curly),
            '<' => Self::Opening(BracketType::Angle),
            ')' => Self::Closing(BracketType::Round),
            ']' => Self::Closing(BracketType::Square),
            '}' => Self::Closing(BracketType::Curly),
            '>' => Self::Closing(BracketType::Angle),
            _ => panic!("Bracket Type unknown"),
        }
    }
}

enum State {
    Corrupt(BracketType, BracketType), // found, expected
    Incomplete(Vec<BracketType>),      // expected
    Ok,
}

fn get_state(data: &Vec<Bracket>) -> State {
    let mut stack: Vec<BracketType> = Vec::new();
    for b in data {
        match b {
            Bracket::Opening(t) => {
                stack.push(*t);
            }
            Bracket::Closing(t) => {
                if let Some(last_opened) = stack.pop() {
                    if last_opened != *t {
                        return State::Corrupt(*t, last_opened);
                    }
                } else {
                    return State::Incomplete(stack);
                }
            }
        }
    }

    if stack.len() != 0 {
        return State::Incomplete(stack);
    }

    State::Ok
}

type Data = Vec<Vec<Bracket>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test10.txt").to_owned())
    }

    fn expected_results() -> (u64, u64) {
        (26397, 288957)
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| v.chars().map(|v| Bracket::new(v)).collect())
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> u64 {
        let mut score = 0;

        for l in data {
            let state = get_state(l);
            match state {
                State::Corrupt(found, _) => {
                    score += found.get_corruption_value();
                }
                _ => {}
            }
        }

        score
    }

    fn two(&self, data: &mut Data) -> u64 {
        let mut scores: Vec<u64> = Vec::new();

        for l in data {
            let mut score = 0;

            let state = get_state(l);
            match state {
                State::Incomplete(expected) => {
                    expected.iter().rev().for_each(|v| {
                        score *= 5;
                        score += v.get_incomplete_value();
                    });
                }
                _ => {}
            }

            if score != 0 {
                scores.push(score);
            }
        }

        scores.sort();

        scores[scores.len() / 2]
    }
}
