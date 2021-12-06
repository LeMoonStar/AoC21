use super::{Day, DayImpl};

const CURRENT_DAY: u8 = 1;

pub struct Data {
    input: Vec<u64>,
}

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data)
    where
        Self: Sized,
    {
        Self::init(&include_str!("test_inputs/test01.txt").to_owned())
    }

    fn expected_results() -> (u64, u64) {
        (7, 5)
    }

    fn init(input: &str) -> (Self, Data)
    where
        Self: Sized,
    {
        (
            Self {},
            Data {
                input: input
                    .lines()
                    .map(|l| l.parse::<u64>().expect("couldnt parse input."))
                    .collect(),
            },
        )
    }

    fn one(&self, data: &mut Data) -> u64 {
        let mut last: u64 = 0;
        let mut count = 0;
        for v in &data.input {
            if last != 0 && *v > last {
                count = count + 1;
            }
            last = *v;
        }
        count
    }

    fn two(&self, data: &mut Data) -> u64 {
        let mut last: u64 = 0;
        let mut count = 0;
        for i in 1..data.input.len() - 1 {
            let sum: u64 = data.input[i - 1] + data.input[i] + data.input[i + 1];

            if i != 1 && sum > last {
                count = count + 1;
            }
            last = sum;
        }
        count
    }
}
