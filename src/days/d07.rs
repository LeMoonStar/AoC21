use super::{Day, DayImpl};

const CURRENT_DAY: u8 = 7;

type Data = Vec<u64>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data)
    where
        Self: Sized,
    {
        Self::init(&include_str!("test_inputs/test07.txt").to_owned())
    }

    fn expected_results() -> (u64, u64) {
        (0, 0)
    }

    fn init(input: &str) -> (Self, Data)
    where
        Self: Sized,
    {
        (
            Self {},
            input
                .lines()
                .map(|v| v.parse::<u64>().expect("error while parsing input."))
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> u64 {
        data.len() as u64
    }

    fn two(&self, data: &mut Data) -> u64 {
        data.len() as u64
    }
}
