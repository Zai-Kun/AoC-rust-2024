use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<()> {
    let file = File::open("days/1.txt")?;
    let reader = BufReader::new(file);

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((num1, num2)) = line.split_once("   ") {
            list1.push(num1.parse().unwrap());
            list2.push(num2.parse().unwrap());
        }
    }

    let mut total = 0;

    for num in list1.iter() {
        let count = list2.iter().filter(|&n| n == num).count();
        total += num*count as i32;
    }

    println!("Day 1 Part 2: {}", total);

    Ok(())
}
