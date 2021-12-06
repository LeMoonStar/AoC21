use super::{Day, DayImpl};
use crate::dprintln;

const CURRENT_DAY: u8 = 6;

pub type Data = [u64; 9];

fn simulate_generation(data: &mut Data, generations: usize) -> u64 {
    for _ in 0..generations {
        let mut new: Data = [0; 9];

        new[0] = data[1];
        new[1] = data[2];
        new[2] = data[3];
        new[3] = data[4];
        new[4] = data[5];
        new[5] = data[6];
        new[6] = data[7] + data[0];
        new[7] = data[8];

        new[8] = data[0];

        *data = new;
        dprintln!("{:?} ({})", data, data.iter().sum::<u64>());
    }
    data.iter().sum()
}

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test06.txt").to_owned())
    }

    fn expected_results() -> (u64, u64) {
        (5934, 26984457539)
    }

    fn init(input: &str) -> (Self, Data) {
        let mut fish: [u64; 9] = [0; 9];

        input.split(',').for_each(|v| {
            let i = v.parse::<usize>().expect("Couldnt parse input.");
            fish[i] = fish[i] + 1;
        });

        (Self {}, fish)
    }

    fn one(&self, data: &mut Data) -> u64 {
        simulate_generation(data, 80)
    }

    fn two(&self, data: &mut Data) -> u64 {
        simulate_generation(data, 256)
    }
}
