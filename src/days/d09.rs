use super::{Day, DayImpl};

const CURRENT_DAY: u8 = 9;

type Data = Vec<u64>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init(input: &String) -> (Self, Data)
    where
        Self: Sized,
    {
        (
            Self {},
            input
                .lines()
                .filter(|v| v.len() != 0)
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
