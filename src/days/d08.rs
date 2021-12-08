use super::{Day, DayImpl};

const CURRENT_DAY: u8 = 8;

#[derive(Clone, Debug, Copy)]
pub enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Segment {
    fn new(c: char) -> Self {
        match c {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            _ => panic!("Not a valid Segment id: {}", c),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum UniqueDigit {
    One,
    Four,
    Seven,
    Eight,
}

impl UniqueDigit {
    fn get_id(&self) -> usize {
        match self {
            Self::One => 0,
            Self::Four => 1,
            Self::Seven => 2,
            Self::Eight => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Digit([bool; 7]);

impl Digit {
    fn new(s: &str) -> Self {
        let mut segments: [bool; 7] = [false; 7];

        s.chars().map(|c| Segment::new(c)).for_each(|v| match v {
            Segment::A => segments[0] = true,
            Segment::B => segments[1] = true,
            Segment::C => segments[2] = true,
            Segment::D => segments[3] = true,
            Segment::E => segments[4] = true,
            Segment::F => segments[5] = true,
            Segment::G => segments[6] = true,
        });

        Self(segments)
    }

    fn get_unique_digit(&self) -> Option<UniqueDigit> {
        let num = if self.0[0] { 1 } else { 0 }
            + if self.0[1] { 1 } else { 0 }
            + if self.0[2] { 1 } else { 0 }
            + if self.0[3] { 1 } else { 0 }
            + if self.0[4] { 1 } else { 0 }
            + if self.0[5] { 1 } else { 0 }
            + if self.0[6] { 1 } else { 0 };

        match num {
            2 => Some(UniqueDigit::One),
            3 => Some(UniqueDigit::Seven),
            4 => Some(UniqueDigit::Four),
            7 => Some(UniqueDigit::Eight),
            _ => None,
        }
    }

    /*fn get_digit(&self, map_info: &MappingInformation) -> u8 {
        map_info.translate_digit(self)
    }*/
}

#[derive(Clone, Copy, Debug)]
pub struct Entry {
    input: [Digit; 10],
    output_digits: [Digit; 4],
}

impl Entry {
    fn new(s: &str) -> Self {
        let parts: Vec<Vec<Digit>> = s
            .split('|')
            .map(|v| v.split_whitespace().map(|v| Digit::new(v)).collect())
            .collect();
        if parts.len() != 2 {
            panic!("there must be two parts per Entry, seperated by a |.");
        }
        if parts[0].len() != 10 {
            panic!(
                "there must be 10 parts in the first part, seperated by whitespaces, found: {}",
                parts[0].len()
            );
        }
        if parts[1].len() != 4 {
            panic!(
                "there must be 4 parts in the second part, seperated by whitespaces, found: {}",
                parts[1].len()
            );
        }
        Self {
            input: parts[0].clone().try_into().unwrap(),
            output_digits: parts[1].clone().try_into().unwrap(),
        }
    }
}

/*#[derive(Clone, Copy, Debug)]
pub struct MappingInformation([[bool; 4]; 7], [[bool; 7]; 9]);

impl MappingInformation {
    fn new() -> Self {
        Self([[false; 4]; 7], [[false; 7]; 9])
    }

    fn tune(&mut self, d: &Digit) {
        if let Some(digit) = d.get_unique_digit() {
            for i in 0..6 {
                if d.0[i] {
                    self.0[i][digit.get_id()] = true;
                }
            }
        }
    }

    fn build(&mut self) {
        let translation: [Segment; 7];

        let

        for i in 0..7 {
            if self.0[i][0] && self.0[i][2]
        }
    }

    fn translate_digit(&self, d: &Digit) -> u8 {
        for i in 0..9 {
            if self.1[i] == d.0 {
                return i as u8;
            }
        }
        0
    }
}*/

type Data = Vec<Entry>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test08.txt").to_owned())
    }

    fn expected_results() -> (u64, u64) {
        (26, 61229)
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.lines().map(|v| Entry::new(v)).collect())
    }

    fn one(&self, data: &mut Data) -> u64 {
        let mut output: u64 = 0;
        for e in data {
            for d in &e.output_digits {
                if let Some(_) = d.get_unique_digit() {
                    output += 1;
                }
            }
        }
        output
    }

    fn two(&self, data: &mut Data) -> u64 {
        /*let mut output: u64 = 0;
        for e in data {
            let mut map_info = MappingInformation::new();
            for d in &e.input {
                map_info.tune(d);
            }
            for d in &e.output_digits {
                map_info.tune(d);
            }

            println!("{:?}", map_info);
            map_info.build();

            for o in &e.output_digits {
                output += o.get_digit(&map_info) as u64;
            }
        }
        output*/
        println!("[PART 2]: Well, I give up, I am too stupid and everybody else managed to do it in less than an hour. Meanwhile I am stuck trying to do this in a overly complicated and stupid way. Don't be like me, dont be dumb... look at the solution sof other people.");
        0
    }
}
