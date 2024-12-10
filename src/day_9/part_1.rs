use std::fs;
use std::io;

pub fn run() -> io::Result<()> {
    let puzzle_input = fs::read_to_string("days/9.txt")?.trim().to_string();
    let mut expanded_input = expand_input(&puzzle_input);

    loop {
        remove_trailing_free_space(&mut expanded_input);
        if !has_free_space(&expanded_input) {
            break;
        }

        let (mut left, right) = split_vec(expanded_input, -1);
        put_in_free_space(&mut left, right);
        expanded_input = left;
    };

    let mut total = 0;
    for (i, c) in expanded_input.iter().enumerate() {
        total += i * (*c as usize);
    }

    println!("Day 9 Part 1: {total}");

    Ok(())
}

fn put_in_free_space(v: &mut Vec<i32>, mut to_put: Vec<i32>) {
    loop {
        if to_put.is_empty() {
            break;
        };

        let (free_start, free_end) = if let Some((fs, fe)) = get_free_space_chunk(&v){
            (fs, fe)
        } else {
            v.extend(to_put);
            break;
        };

        let free_space = free_end - free_start;

        let (prefix, to_put_part) = to_put.split_at(to_put.len().saturating_sub(free_space));
        let _to_put: Vec<i32> = to_put_part.to_vec();
        to_put = prefix.to_vec();

        for (i, c) in _to_put.iter().rev().enumerate() {
            v[free_start + i] = *c;
        }
    }
}

fn expand_input(s: &str) -> Vec<i32> {
    let mut rearranged: Vec<i32> = Vec::new();

    let mut file_id = 0;
    for (i, c) in s.chars().enumerate() {
        let parsed_c: usize = c.to_string().parse().unwrap();
        if i % 2 == 0 {
            for _ in 0..parsed_c {
                rearranged.push(file_id as i32);
            }
            file_id += 1;
        } else {
            for _ in 0..parsed_c {
                rearranged.push(-1);
            }
        }
    }

    rearranged
}

fn split_vec(vec: Vec<i32>, _c: i32) -> (Vec<i32>, Vec<i32>) {
    if let Some(pos) = vec.iter().rposition(|&c| c == _c) {
        return (vec[0..pos + 1].to_vec(), vec[pos + 1..].to_vec());
    }
    (vec, Vec::new())
}

fn get_free_space_chunk(v: &Vec<i32>) -> Option<(usize, usize)> {
    let mut start = 0;
    for i in 0..v.len() {
        if v[i] != -1 {
            if start != 0 {
                return Some((start, i));
            }
        } else {
            if start == 0 {
                start = i;
            }
        }
    }
    if start != 0 {
        return Some((start, v.len()));
    }
    None
}

fn has_free_space(v: &Vec<i32>) -> bool {
    for c in v {
        if *c == -1 {
            return true;
        }
    }
    false
}

fn remove_trailing_free_space(vec: &mut Vec<i32>) {
    while vec.last() == Some(&-1) {
        vec.pop();
    }
}
