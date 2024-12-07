use std::str::FromStr;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<()> {
    let file = File::open("days/7.txt")?;
    let reader = BufReader::new(file);
    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();
        let test_value = usize::from_str(parts[0]).expect("Invalid test value");
        let operands: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|x| usize::from_str(x).expect("Invalid operand"))
            .collect();

        if is_valid(test_value, &operands) {
            total_sum += test_value;
        }
    }

    println!("Day 7 Part 2: {}", total_sum);
    Ok(())
}

fn is_valid(test_value: usize, operands: &[usize]) -> bool {
    let num_gaps = operands.len() - 1;
    let total_combinations = 3_usize.pow(num_gaps as u32);

    for i in 0..total_combinations {
        let mut result = operands[0];
        let mut current_operator_combination = i;

        for j in 0..num_gaps {
            let operator_index = current_operator_combination % 3;
            current_operator_combination /= 3;

            match operator_index {
                0 => result += operands[j + 1],
                1 => result *= operands[j + 1],
                2 => result = (result.to_string() + &operands[j+1].to_string()).parse().unwrap(),
                _ => unreachable!(),
            }
        }

        if result == test_value {
            return true;
        }
    }

    false
}
