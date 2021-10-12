use super::Day;

pub struct Day1 {
    input: String,
}

impl Day for Day1 {
    
    fn new(input: &String) -> Self {
        Day1 {
            input: input.clone()
        }
    }

    fn first(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(self.input.parse::<u64>()?)
    }

    fn second(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(self.input.parse::<u64>()?)
    }
}
