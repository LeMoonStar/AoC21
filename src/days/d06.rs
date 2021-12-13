use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 6;

pub type Data = [u64; 9];

fn simulate_generation(data: &mut Data, generations: usize) -> u64 {
    for _ in 0..generations {
        let new = data[0];

        data[0] = data[1];
        data[1] = data[2];
        data[2] = data[3];
        data[3] = data[4];
        data[4] = data[5];
        data[5] = data[6];
        data[6] = data[7];
        data[7] = data[8];

        data[8] = new;
        data[6] += new;
    }
    data.iter().sum()
}

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test06.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(5934), Answer::Number(26984457539))
    }

    fn init(input: &str) -> (Self, Data) {
        let mut fish: [u64; 9] = [0; 9];

        input.split(',').for_each(|v| {
            let i = v.parse::<usize>().expect("Couldnt parse input.");
            fish[i] = fish[i] + 1;
        });

        (Self {}, fish)
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(simulate_generation(data, 80))
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(simulate_generation(data, 256))
    }
}
