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

    list1.sort_unstable();
    list2.sort_unstable();

    let total: i32 = list1.iter()
        .zip(list2.iter())
        .map(|(&num1, &num2)| (num1 - num2).abs())
        .sum();

    println!("Total: {}", total);

    Ok(())
}
