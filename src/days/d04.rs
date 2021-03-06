use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 4;

#[derive(Debug, Clone)]
struct Field {
    num: u8,
    marked: bool,
}

#[derive(Debug, Clone)]
struct Board {
    data: Vec<Vec<Field>>,
}

#[derive(Debug, Clone)]
pub struct Data {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl Board {
    fn new<S: AsRef<str>>(input: &S) -> Self {
        let values: Vec<Vec<Field>> = input
            .as_ref()
            .lines()
            .map(|v| {
                v.split_whitespace()
                    .map(|v| Field {
                        num: v.parse::<u8>().unwrap_or(0),
                        marked: false,
                    })
                    .collect()
            })
            .collect();

        if values.len() != values.get(0).expect("Failed parsing").len() {
            panic!("couldnt parse board")
        }

        Board { data: values }
    }

    fn mark_nums(&mut self, num: u8) {
        for l in &mut self.data {
            for f in l {
                if f.num == num {
                    f.marked = true;
                }
            }
        }
    }

    fn is_completed(&self) -> bool {
        for line in 0..5 {
            let mut completed = true;
            for collumn in 0..5 {
                if !self.data[line][collumn].marked {
                    completed = false;
                    break;
                }
            }

            if completed {
                return true;
            }
        }

        for collumn in 0..5 {
            let mut completed = true;
            for line in 0..5 {
                if !self.data[line][collumn].marked {
                    completed = false;
                    break;
                }
            }

            if completed {
                return true;
            }
        }

        false
    }

    fn calculate_score(&self, num: u8) -> u64 {
        let mut sum: u64 = 0;
        for l in &self.data {
            for f in l {
                if !f.marked {
                    sum += f.num as u64;
                }
            }
        }

        sum * num as u64
    }
}

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test04.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(4512), Answer::Number(1924))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            Data {
                numbers: input
                    .lines()
                    .nth(0)
                    .unwrap()
                    .split(",")
                    .map(|v| v.parse::<u8>().expect("couldnt parse input"))
                    .collect(),
                boards: input
                    .split("\n\n")
                    .skip(1)
                    .filter(|v| v.len() != 0)
                    .map(|v| Board::new(&v))
                    .collect(),
            },
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut boards = data.boards.clone();

        let mut result = 0;
        for n in &data.numbers {
            for b in &mut boards {
                if result != 0 {
                    break;
                }
                b.mark_nums(*n);
                if b.is_completed() {
                    result = b.calculate_score(*n);
                    break;
                }
            }
        }

        Answer::Number(result)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut boards: Vec<Board> = data.boards.clone();
        let mut result = 0;
        for n in &data.numbers {
            for b in &mut boards {
                if result != 0 {
                    break;
                }
                b.mark_nums(*n);
            }
            if boards.len() == 1 {
                if boards[0].is_completed() {
                    result = boards[0].calculate_score(*n);
                    break;
                }
            } else {
                boards = boards.into_iter().filter(|v| !v.is_completed()).collect();
            }
        }

        Answer::Number(result)
    }
}
