use std::fs;
use std::io;

pub fn run() -> io::Result<()> {
    let mut total_xmas = 0;
    let raw_file = fs::read_to_string("days/4.txt")?;
    total_xmas += count_xmas(&raw_file);

    let matrix = raw_file
        .trim()
        .split("\n")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    total_xmas += count_xmas(&flip_90_degrees(&matrix));

    for (i, row) in matrix.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch != 'X' {
                continue;
            }
            //left-up
            if i >= 3 && j >= 3 {
                let (c1, c2, c3) = (
                    matrix[i - 1][j - 1],
                    matrix[i - 2][j - 2],
                    matrix[i - 3][j - 3],
                );
                if c1 == 'M' && c2 == 'A' && c3 == 'S' {
                    total_xmas += 1;
                }
            }

            //left down
            if (matrix.len() - 1) - i >= 3 && j >= 3 {
                let (c1, c2, c3) = (
                    matrix[i + 1][j - 1],
                    matrix[i + 2][j - 2],
                    matrix[i + 3][j - 3],
                );
                if c1 == 'M' && c2 == 'A' && c3 == 'S' {
                    total_xmas += 1;
                }
            }

            //right up
            if i >= 3 && (matrix[0].len() - 1) - j >= 3 {
                let (c1, c2, c3) = (
                    matrix[i - 1][j + 1],
                    matrix[i - 2][j + 2],
                    matrix[i - 3][j + 3],
                );
                if c1 == 'M' && c2 == 'A' && c3 == 'S' {
                    total_xmas += 1;
                }
            }
            //right down
            if (matrix.len() - 1) - i >= 3 && (matrix[0].len() - 1) - j >= 3 {
                let (c1, c2, c3) = (
                    matrix[i + 1][j + 1],
                    matrix[i + 2][j + 2],
                    matrix[i + 3][j + 3],
                );
                if c1 == 'M' && c2 == 'A' && c3 == 'S' {
                    total_xmas += 1;
                }
            }
        }
    }
    println!("Day 4 Part 1: {total_xmas}");
    Ok(())
}

fn count_xmas(s: &str) -> usize {
    s.to_lowercase().matches("xmas").count() + s.to_lowercase().matches("samx").count()
}

fn flip_90_degrees(matrix: &Vec<Vec<char>>) -> String {
    let mut new = String::new();

    for i in 0..matrix[0].len() {
        for j in 0..matrix.len() {
            new += &matrix[matrix.len() - 1 - j][i].to_string();
        }
        new += "\n"
    }

    new
}
