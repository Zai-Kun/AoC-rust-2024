use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<()> {
    let file = File::open("days/2.txt")?;
    let reader = BufReader::new(file);

    let mut total_safe = 0;

    for line in reader.lines() {
        let line = line?;
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        if is_safe(&levels) {
            total_safe += 1;
        }
    }

    println!("Total safe: {}", total_safe);

    Ok(())
}

// all increasing or decreasing by at least 1 and at most 3
fn is_safe(levels: &[i32]) -> bool {
    let mut inc: i8 = -1;

    for i in 0..levels.len() {
        if i + 1 == levels.len() {
            break;
        }

        let (n1, n2) = (levels[i], levels[i + 1]);

        let distance = (n1 - n2).abs();
        if distance == 0 || distance > 3 {
            return false;
        }

        if i == 0 {
            inc = if n1 > n2 { 1 } else { 0 };
        } else {
            if (inc == 1 && n1 < n2) || (inc == 0 && n1 > n2) {
                return false;
            }
        }
    }

    true
}
