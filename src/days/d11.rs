use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 11;

fn get_pos(data: &mut Data, pos: (usize, usize), offset: (isize, isize)) -> Option<&mut u8> {
    let new_pos = (pos.0 as isize + offset.0, pos.1 as isize + offset.1);
    if new_pos.0 < 0 || new_pos.1 < 0 {
        return None;
    }

    let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

    match data.get_mut(new_pos.0) {
        Some(r) => r.get_mut(new_pos.1),
        None => None,
    }
}

fn flash(data: &mut Data, pos: (usize, usize), offset: (isize, isize), by_neigbour: bool) -> u64 {
    if let Some(this) = get_pos(data, pos, offset) {
        if *this == 0 {
            return 0;
        }
        if by_neigbour {
            *this += 1;
        }

        if *this > 9 {
            *this = 0;
            1 + flash(data, pos, (offset.0 + -1, offset.1 + -1), true)
                + flash(data, pos, (offset.0 + -1, offset.1 + 1), true)
                + flash(data, pos, (offset.0 + 1, offset.1 + -1), true)
                + flash(data, pos, (offset.0 + 1, offset.1 + 1), true)
                + flash(data, pos, (offset.0 + -1, offset.1 + 0), true)
                + flash(data, pos, (offset.0 + 1, offset.1 + 0), true)
                + flash(data, pos, (offset.0 + 0, offset.1 + -1), true)
                + flash(data, pos, (offset.0 + 0, offset.1 + 1), true)
        } else {
            0
        }
    } else {
        0
    }
}

fn simulate(data: &mut Data, steps: usize) -> u64 {
    let mut flashes = 0;

    for _ in 0..steps {
        let mut buf = data.clone();
        for row in 0..buf.len() {
            for col in 0..buf[row as usize].len() {
                buf[row][col] += 1;
            }
        }

        for row in 0..buf.len() {
            for col in 0..buf[row as usize].len() {
                flashes += flash(&mut buf, (row, col), (0, 0), false);
            }
        }

        *data = buf;
    }

    flashes
}

type Data = Vec<Vec<u8>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test11.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(1656), Answer::Number(195))
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
        Answer::Number(simulate(data, 100))
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut steps = 0;

        loop {
            steps += 1;
            let flashes = simulate(data, 1);
            if flashes == (data.len() * data[0].len()) as u64 {
                break;
            }
        }

        Answer::Number(steps)
    }
}
