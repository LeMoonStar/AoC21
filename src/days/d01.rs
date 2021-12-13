use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 1;

pub type Data = Vec<u64>;

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test01.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(7), Answer::Number(5))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|l| l.parse::<u64>().expect("couldnt parse input."))
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut last: u64 = 0;
        let mut count = 0;
        for v in data {
            if last != 0 && *v > last {
                count = count + 1;
            }
            last = *v;
        }
        Answer::Number(count)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut last: u64 = 0;
        let mut count = 0;
        for i in 1..data.len() - 1 {
            let sum: u64 = data[i - 1] + data[i] + data[i + 1];

            if i != 1 && sum > last {
                count = count + 1;
            }
            last = sum;
        }
        Answer::Number(count)
    }
}
