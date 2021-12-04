use super::{Day, DayImpl};

const CURRENT_DAY: u8 = 14;

type Data = Vec<u64>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn initTest() -> (Self, Data)
    where
        Self: Sized,
    {
        Self::init(&"test_inputs/test14.txt".to_owned())
    }

    fn expected_results() -> (u64, u64) {
        (0, 0)
    }
    
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
