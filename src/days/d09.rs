use super::{Answer, Day, DayImpl};
use crate::vprintln;

const CURRENT_DAY: u8 = 9;

fn get_value_of_pos(d: &Data, p: (i32, i32)) -> u8 {
    if p.0 < 0 || p.1 < 0 {
        return u8::MAX;
    }

    if let Some(r) = d.get(p.0 as usize) {
        if let Some(v) = r.get(p.1 as usize) {
            *v
        } else {
            u8::MAX
        }
    } else {
        u8::MAX
    }
}

enum MapType {
    HighPoint,
    Normal(bool),
}

fn get_basin_size(d: &mut Vec<Vec<MapType>>, p: (i32, i32)) -> u64 {
    if p.0 < 0 || p.1 < 0 {
        return 0;
    }

    if let Some(r) = d.get_mut(p.0 as usize) {
        if let Some(v) = r.get_mut(p.1 as usize) {
            match v {
                MapType::HighPoint => 0,
                MapType::Normal(walked) => {
                    if *walked {
                        0
                    } else {
                        *v = MapType::Normal(true);

                        let size = 1
                            + get_basin_size(d, (p.0 + 1, p.1))
                            + get_basin_size(d, (p.0 - 1, p.1))
                            + get_basin_size(d, (p.0, p.1 + 1))
                            + get_basin_size(d, (p.0, p.1 - 1));

                        size
                    }
                }
            }
        } else {
            0
        }
    } else {
        0
    }
}

type Data = Vec<Vec<u8>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test09.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(15), Answer::Number(1134))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| {
                    v.chars()
                        .map(|v| v.to_digit(10).expect("error while parsing input.") as u8)
                        .collect()
                })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut risk_sum = 0;
        for row in 0..data.len() as i32 {
            for col in 0..data[row as usize].len() as i32 {
                if data[row as usize][col as usize] < get_value_of_pos(&data, (row - 1, col))
                    && data[row as usize][col as usize] < get_value_of_pos(&data, (row + 1, col))
                    && data[row as usize][col as usize] < get_value_of_pos(&data, (row, col - 1))
                    && data[row as usize][col as usize] < get_value_of_pos(&data, (row, col + 1))
                {
                    risk_sum += data[row as usize][col as usize] as u64 + 1;
                }
            }
        }

        Answer::Number(risk_sum)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut map: Vec<Vec<MapType>> = data
            .iter()
            .map(|v| {
                v.iter()
                    .map(|v| {
                        if *v != 9 {
                            MapType::Normal(false)
                        } else {
                            MapType::HighPoint
                        }
                    })
                    .collect()
            })
            .collect();

        let mut basins: Vec<u64> = Vec::new();

        for row in 0..data.len() as i32 {
            for col in 0..data[row as usize].len() as i32 {
                if data[row as usize][col as usize] < get_value_of_pos(&data, (row - 1, col))
                    && data[row as usize][col as usize] < get_value_of_pos(&data, (row + 1, col))
                    && data[row as usize][col as usize] < get_value_of_pos(&data, (row, col - 1))
                    && data[row as usize][col as usize] < get_value_of_pos(&data, (row, col + 1))
                {
                    basins.push(get_basin_size(&mut map, (row, col)));
                }
            }
        }

        basins.sort();
        vprintln!("Found {} basins : {:?}", basins.len(), basins);

        if basins.len() < 3 {
            panic!("less than 3 basins.")
        }

        Answer::Number(
            basins[basins.len() - 1] * basins[basins.len() - 2] * basins[basins.len() - 3],
        )
    }
}
