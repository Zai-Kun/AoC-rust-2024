use std::fs;
use std::io;

pub fn run() -> io::Result<()> {
    let puzzel_input = fs::read_to_string("days/8.txt")?;
    let matrix = puzzel_input
        .trim()
        .split("\n")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut all_antinodes: Vec<(usize, usize)> = vec![];

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let c = matrix[row][col];
            if c != '.' {
                find_all_antennas_n_antinodes(&matrix, &mut all_antinodes, (row, col));
            }
        }
    }

    println!("Day 8 Part 1: {}", all_antinodes.len());

    Ok(())
}

fn find_all_antennas_n_antinodes(
    matrix: &Vec<Vec<char>>,
    all_antinodes: &mut Vec<(usize, usize)>,
    antenna_cords: (usize, usize),
) {
    let to_find = matrix[antenna_cords.0][antenna_cords.1];
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let c = matrix[row][col];
            if c == to_find && (row, col) != antenna_cords {
                let row_dist = (row as i32) - antenna_cords.0 as i32;
                let col_dist = (col as i32) - antenna_cords.1 as i32;

                let antinode_cord = (row as i32 + row_dist, col as i32 + col_dist);
                if !cords_out_of_bounds(
                    matrix,
                    (antinode_cord.0 as usize, antinode_cord.1 as usize),
                ) && !all_antinodes
                    .contains(&(antinode_cord.0 as usize, antinode_cord.1 as usize))
                {
                    all_antinodes.push((antinode_cord.0 as usize, antinode_cord.1 as usize));
                }
            }
        }
    }
}

fn cords_out_of_bounds(matrix: &Vec<Vec<char>>, cords: (usize, usize)) -> bool {
    cords.0 >= matrix.len() || cords.1 >= matrix[cords.0].len()
}
