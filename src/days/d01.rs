use super::Day;

pub struct Day1;

impl Day for Day1 {
    fn first(input: &String) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(input.parse::<u64>()?)
    }

    fn second(input: &String) -> Result<u64, Box<dyn std::error::Error>> {     
        Ok(input.parse::<u64>()?)
    }
}