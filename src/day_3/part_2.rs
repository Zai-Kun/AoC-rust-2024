use regex::Regex;
use std::fs;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let text = String::from_utf8(fs::read("days/3.txt")?)?;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut total = 0;

    for captures in re.captures_iter(&text) {
        let starting_index = captures.get(0).unwrap().start();
        let num1: i32 = captures.get(1).unwrap().as_str().parse()?;
        let num2: i32 = captures.get(2).unwrap().as_str().parse()?;

        let last_dont = &text[0..starting_index].rfind("don't()");
        if let Some(last_dont_i) = last_dont{
            let last_do = &text[0..starting_index].rfind("do()");
            if let Some(last_do_i) = last_do{
                if last_do_i > last_dont_i{
                    total += num1*num2;
                }
            }
        } else {
            total += num1*num2;
        }
    }

    println!("Day 3 Part 2: {}", total);
    Ok(())
}
