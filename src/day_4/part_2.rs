use std::fs;
use std::io;

pub fn run() -> io::Result<()> {
    let raw_file = fs::read_to_string("days/4.txt")?;
    let matrix = raw_file
        .trim()
        .split("\n")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut total_xmas = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch != 'A'
                || !(i >= 1 && j >= 1)
                || !((matrix.len() - 1) - i >= 1 && j >= 1)
                || !(i >= 1 && (matrix[0].len() - 1) - j >= 1)
                || !((matrix.len() - 1) - i >= 1 && (matrix[0].len() - 1) - j >= 1)
            {
                continue;
            }

            //left-up, right-down
            let c1 = matrix[i-1][j-1];
            let c2 = matrix[i+1][j+1];
            if !(c1 == 'S' && c2 == 'M') && !(c1 == 'M' && c2 == 'S'){
                continue
            }

            //left-down, right-up
            let c1 = matrix[i+1][j-1];
            let c2 = matrix[i-1][j+1];
            if !(c1 == 'S' && c2 == 'M') && !(c1 == 'M' && c2 == 'S'){
                continue
            }

            total_xmas += 1;
        }
    }
    println!("Day 4 Part 2: {total_xmas}");
    Ok(())
}
