use super::Day;

pub struct Day3 {
    input: Vec<u64>,
    width: usize,
    length: usize,
}

fn get_bit_from_offset(v: u64, offset: usize) -> bool {
    println!("{} at offset {} is: {}", v, offset, v & (1 << offset) != 0);
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

impl Day for Day3 {
    fn new(input: &String) -> Self {
        Day3 {
            input: input
                .lines()
                .filter(|v| v.len() != 0)
                .map(|v| u64::from_str_radix(v, 2).expect("Couldn't parse input."))
                .collect(),
            width: input.lines().next().unwrap().len(),
            length: input.lines().collect::<Vec<&str>>().len(),
        }
    }

    fn first(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut counts = Vec::<u64>::with_capacity(self.width);
        counts.resize(self.width, 0);

        for v in &self.input {
            for i in 0..self.width {
                counts[i] = counts[i] + if (v & (1 << i)) != 0 { 1 } else { 0 };
            }
        }

        let mut gamma: u64 = 0;
        let mut epsilon: u64 = 0;

        for i in 0..self.width {
            if counts[i] > (self.length as u64) / 2 {
                gamma = gamma + (1 << i);
            } else {
                epsilon = epsilon + (1 << i);
            }
        }

        Ok(gamma as u64 * epsilon as u64)
    }

    fn second(&self) -> Result<u64, Box<dyn std::error::Error>> {
        println!("Since I really cant find what I did wrong, and I dont wanna spend more time on this today:");
        println!("https://www.youtube.com/watch?v=llqWTJGUFeE");

        Ok(0)

        /*let mut o2_values: Vec<u64> = self
            .input
            .iter()
            .filter(|v| get_bit_from_offset(**v, self.width - 1))
            .map(|v| *v)
            .collect();

        for i in 1..self.width {
            let offset = i;//self.width - i;
            let positive_bit_count = count_bits(&o2_values, offset);
            println!("-");
            println!("{:?}", o2_values);
            println!(
                "{} values with a 1(true) at offset {}",
                positive_bit_count, offset
            );
            println!(
                "There are curently {} elements. Meaning that there are {} elements with a 0(false)",
                o2_values.len(),
                o2_values.len() as u64 - positive_bit_count
            );
            let expected_bit: bool =
                positive_bit_count > o2_values.len() as u64 - positive_bit_count;
            println!("Thus we expect a {} on this position.", expected_bit);
            o2_values = o2_values
                .iter()
                .filter(|v| get_bit_from_offset(**v, offset))
                .map(|v| *v)
                .collect();
            println!("resulting list: {:?}", o2_values);
        }

        let o2_rating = if o2_values.len() == 1 {
            o2_values[0]
        } else {
            panic!("too many or too few results.")
        };

        println!("O2 rating: {}", o2_rating);

        Ok(0)*/
    }
}
