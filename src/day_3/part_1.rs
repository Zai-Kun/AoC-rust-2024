use regex::Regex;
use std::fs;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let text = String::from_utf8(fs::read("days/3.txt")?)?;
    
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut total = 0;
    
    for captures in re.captures_iter(&text) {
        let num1: i32 = captures.get(1).unwrap().as_str().parse()?;
        let num2: i32 = captures.get(2).unwrap().as_str().parse()?;
        total += num1*num2;
    }

    println!("Day 3 Part 1: {}", total);

    Ok(())
}
