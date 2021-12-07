use super::{Day, DayImpl};

const CURRENT_DAY: u8 = 7;

fn add_with_all_lower(input: u64) -> u64 {
    let mut output = 0;
    for j in 1..input + 1 {
        output += j;
    }

    output
}

type Data = Vec<u64>;

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test07.txt").to_owned())
    }

    fn expected_results() -> (u64, u64) {
        (37, 168)
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .split(',')
                .map(|v| v.parse().expect("Couldn't parse input"))
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> u64 {
        let min = *data.iter().min().unwrap();
        let max = *data.iter().max().unwrap();

        let mut least_fuel = u64::MAX;

        for i in min..max {
            least_fuel = least_fuel.min(data.iter().map(|v| v.abs_diff(i)).sum())
        }

        least_fuel
    }

    fn two(&self, data: &mut Data) -> u64 {
        let min = *data.iter().min().unwrap();
        let max = *data.iter().max().unwrap();

        let mut least_fuel = u64::MAX;

        for i in min..max {
            least_fuel =
                least_fuel.min(data.iter().map(|v| add_with_all_lower(v.abs_diff(i))).sum())
        }

        least_fuel
    }
}
