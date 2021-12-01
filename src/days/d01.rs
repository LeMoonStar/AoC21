use super::Day;

pub struct Day1 {
    input: Vec<u64>,
}

impl Day for Day1 {
    fn new(input: &String) -> Self {
        Day1 {
            input: input
                .lines()
                .map(|l| l.parse::<u64>().expect("couldnt parse input."))
                .collect(),
        }
    }

    fn first(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut last: u64 = 0;
        let mut count = 0;
        for v in &self.input {
            if last != 0 && *v > last {
                count = count + 1;
            }
            last = *v;
        }
        Ok(count)
    }

    fn second(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut last: u64 = 0;
        let mut count = 0;
        for i in 1..self.input.len() - 1 {
            let sum: u64 = self.input[i - 1] + self.input[i] + self.input[i + 1];

            if i != 1 && sum > last {
                count = count + 1;
            }
            last = sum;
        }
        Ok(count)
    }
}
