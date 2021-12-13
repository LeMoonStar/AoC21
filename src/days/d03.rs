use super::{Answer, Day, DayImpl};
use crate::dprintln;

const CURRENT_DAY: u8 = 3;

fn get_bit_from_offset(v: u64, offset: usize) -> bool {
    dprintln!("{} at offset {} is: {}", v, offset, v & (1 << offset) != 0);
    v & (1 << offset) != 0
}

fn count_bits(vec: &Vec<u64>, offset: usize) -> u64 {
    let mut count = 0;
    for v in vec {
        if get_bit_from_offset(*v, offset) {
            count = count + 1;
        }
    }
    count
}

#[derive(Clone, Debug)]
pub struct Data {
    input: Vec<u64>,
    width: usize,
    length: usize,
}

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test03.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(198), Answer::Number(230))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            Data {
                input: input
                    .lines()
                    .map(|v| u64::from_str_radix(v, 2).expect("Couldn't parse input."))
                    .collect(),
                width: input.lines().next().unwrap().len(),
                length: input.lines().collect::<Vec<&str>>().len(),
            },
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut counts = Vec::<u64>::with_capacity(data.width);
        counts.resize(data.width, 0);

        for v in &data.input {
            for i in 0..data.width {
                counts[i] = counts[i] + if (v & (1 << i)) != 0 { 1 } else { 0 };
            }
        }

        let mut gamma: u64 = 0;
        let mut epsilon: u64 = 0;

        for i in 0..data.width {
            if counts[i] > (data.length as u64) / 2 {
                gamma = gamma + (1 << i);
            } else {
                epsilon = epsilon + (1 << i);
            }
        }

        Answer::Number(gamma as u64 * epsilon as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut o2_values: Vec<u64> = data
            .input
            .iter()
            .filter(|v| get_bit_from_offset(**v, data.width - 1))
            .map(|v| *v)
            .collect();

        for i in 1..data.width {
            let offset = i; //self.width - i;
            let positive_bit_count = count_bits(&o2_values, offset);
            dprintln!("-");
            dprintln!("{:?}", o2_values);
            dprintln!(
                "{} values with a 1(true) at offset {}",
                positive_bit_count,
                offset
            );
            dprintln!(
                "There are curently {} elements. Meaning that there are {} elements with a 0(false)",
                o2_values.len(),
                o2_values.len() as u64 - positive_bit_count
            );
            let expected_bit: bool =
                positive_bit_count > o2_values.len() as u64 - positive_bit_count;
            dprintln!("Thus we expect a {} on this position.", expected_bit);
            o2_values = o2_values
                .iter()
                .filter(|v| get_bit_from_offset(**v, offset))
                .map(|v| *v)
                .collect();
            dprintln!("resulting list: {:?}", o2_values);
        }

        let o2_rating = if o2_values.len() == 1 {
            o2_values[0]
        } else {
            panic!("too many or too few results.")
        };

        dprintln!("O2 rating: {}", o2_rating);

        Answer::Number(o2_rating)
    }
}
