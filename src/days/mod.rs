use crate::DayMode;

pub trait Day {
    fn first(input: &String) -> Result<u64, Box<dyn std::error::Error>>;
    fn second(input: &String) -> Result<u64, Box<dyn std::error::Error>>;
    
    fn run(input: &String, mode: &DayMode) -> Vec<Result<u64, Box<dyn std::error::Error>>> {
        let mut output = Vec::with_capacity(match mode {
            DayMode::One | DayMode::Two => 1,
            DayMode::Both => 2
        });
        match mode {
            DayMode::One => output.push(Self::first(input)),
            DayMode::Two => output.push(Self::second(input)),
            DayMode::Both => {output.push(Self::first(input)); output.push(Self::second(input))},
        };
        output
    }
}

pub mod d01;
//pub mod d02;
//pub mod d03;
//pub mod d04;
//pub mod d05;
//pub mod d06;
//pub mod d07;
//pub mod d08;
//pub mod d09;
//pub mod d10;
//pub mod d11;
//pub mod d12;
//pub mod d13;
//pub mod d14;
//pub mod d15;
//pub mod d16;
//pub mod d17;
//pub mod d18;
//pub mod d19;
//pub mod d20;
//pub mod d21;
//pub mod d22;
//pub mod d23;
//pub mod d24;
//pub mod d25;
