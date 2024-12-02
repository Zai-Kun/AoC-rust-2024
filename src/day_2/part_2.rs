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

fn is_safe(levels: &[i32]) -> bool {
    let (safe, index1, index2) = _is_safe(levels);
    if safe {
        return true;
    }

    let mut indices = vec![index1, index2];
    if index1 > 0 {
        indices.insert(0, index1 - 1);
    }

    for i in indices {
        let without_i = [&levels[..i], &levels[i + 1..]].concat();
        let (safe, _, _) = _is_safe(&without_i);
        if safe {
            return true;
        }
    }

    false
}


// all increasing or decreasing by at least 1 and at most 3
fn _is_safe(levels: &[i32]) -> (bool, usize, usize) {
    let mut trend: Option<bool> = None;

    for i in 0..levels.len() - 1 {
        let (n1, n2) = (levels[i], levels[i + 1]);
        let distance = (n1 - n2).abs();

        if distance == 0 || distance > 3 {
            return (false, i, i + 1);
        }

        let current_trend = n1 > n2;

        if let Some(previous_trend) = trend {
            if current_trend != previous_trend {
                return (false, i, i + 1);
            }
        } else {
            trend = Some(current_trend);
        }
    }

    (true, 0, 0)
}

