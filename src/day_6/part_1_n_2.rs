use std::fs;
use std::io;
use std::collections::HashMap;

pub fn run() -> io::Result<()> {
    let puzzel_input = fs::read_to_string("days/6.txt")?;
    let mut matrix = puzzel_input
        .trim()
        .split("\n")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (initial_row, initial_col, initial_direction) =
        guard_position(&matrix).expect("The hell, the gaurd has been lost in the backrooms :<");

    matrix[initial_row][initial_col] = 'x';
    let guard_path =
        run_simulation(&matrix, initial_row, initial_col, initial_direction).expect("The guard is looping");

    println!("Day 6 Part 1: {}", guard_path.len());

    let mut total_available_loops = 0;

    for (row, col) in guard_path{
        if (row, col) == (initial_row, initial_col){
            continue;
        }
        matrix[row][col] = '#';
        if let None = run_simulation(&matrix, initial_row, initial_col, initial_direction){
            total_available_loops += 1;
        }
        matrix[row][col] = '.';
    }

    println!("Day 6 Part 2: {total_available_loops}");

    Ok(())
}

// returns None if the guard is looping
fn run_simulation(
    matrix: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    direction: char,
) -> Option<Vec<(usize, usize)>> {
    let mut current_direction = direction;
    let mut row = row;
    let mut col = col;
    let mut guard_path = HashMap::new();
    guard_path.insert((row, col), current_direction);

    loop {
        let (didnt_end, hit_wall, new_direction, new_pos) =
            check_position(&matrix, row, col, current_direction);
        if !didnt_end {
            break;
        }
        if hit_wall {
            current_direction = new_direction;
            continue;
        }
        (row, col) = new_pos;

        if let Some(&direction) = guard_path.get(&(row, col)) {
            if direction == current_direction {
                return None;
            }
        } else {
            guard_path.insert((row, col), current_direction);
        }
    }

    Some(guard_path.keys().cloned().collect())
}

fn check_position(
    matrix: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    direction: char,
) -> (bool, bool, char, (usize, usize)) {
    if direction == '^' {
        if row > 0 {
            if matrix[row - 1][col] == '#' {
                return (true, true, '>', (row, col));
            }
            return (true, false, direction, (row - 1, col));
        }
    } else if direction == '>' {
        if (matrix[row].len() - 1) - col > 0 {
            if matrix[row][col + 1] == '#' {
                return (true, true, 'v', (row, col));
            }
            return (true, false, direction, (row, col + 1));
        }
    } else if direction == 'v' {
        if (matrix.len() - 1) - row > 0 {
            if matrix[row + 1][col] == '#' {
                return (true, true, '<', (row, col));
            }
            return (true, false, direction, (row + 1, col));
        }
    } else if direction == '<' {
        if col > 0 {
            if matrix[row][col - 1] == '#' {
                return (true, true, '^', (row, col));
            }
            return (true, false, direction, (row, col - 1));
        }
    }

    return (false, false, direction, (row, col));
}

fn guard_position(matrix: &Vec<Vec<char>>) -> Option<(usize, usize, char)> {
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let c = matrix[row][col];
            if c == '^' || c == 'v' || c == '<' || c == '>' {
                return Some((row, col, c));
            }
        }
    }
    None
}
