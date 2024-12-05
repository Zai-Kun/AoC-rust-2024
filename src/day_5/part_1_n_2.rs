use std::fs::{self};
use std::io;

pub struct SortByRules {
    rules: Vec<(i32, i32)>,
}

impl SortByRules {
    pub fn new(rules: Vec<(i32, i32)>) -> Self {
        Self { rules }
    }

    pub fn is_sorted(&self, list: &Vec<i32>) ->bool{
        let filtered: Vec<_> = self
            .rules
            .iter()
            .filter(|(n1, n2)| list.contains(n1) && list.contains(n2))
            .collect();
        for (n1, n2) in &filtered {
            let i1 = list.iter().position(|x| x == n1).unwrap();
            let i2 = list.iter().position(|x| x == n2).unwrap();
            if i1 > i2 {
                return false;
            }
        };

        return true;
    }

    pub fn sort(&self, list: &mut Vec<i32>) {
        let filtered: Vec<_> = self
            .rules
            .iter()
            .filter(|(n1, n2)| list.contains(n1) && list.contains(n2))
            .collect();

        let mut sorted = false;
        while !sorted {
            sorted = true;
            for (n1, n2) in &filtered {
                let i1 = list.iter().position(|x| x == n1).unwrap();
                let i2 = list.iter().position(|x| x == n2).unwrap();
                if i1 > i2 {
                    sorted = false;
                    let r = list.remove(i2);
                    list.insert(i1, r);
                }
            }
        }
    }
}

pub fn run() -> io::Result<()> {
    let file = fs::read_to_string("days/5.txt")?;
    let (rules, lists) = file.split_once("\n\n").unwrap();
    let rules: Vec<(i32, i32)> = rules
        .lines()
        .map(|line| {
            let (n1, n2) = line.split_once("|").unwrap();
            (n1.trim().parse().unwrap(), n2.trim().parse().unwrap())
        })
        .collect();
    let mut lists: Vec<Vec<i32>> = lists
        .lines()
        .map(|line| {
            line.trim()
                .split(",")
                .map(|n| n.trim().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let sort = SortByRules::new(rules);

    let mut part_1_total = 0;
    let mut part_2_total = 0;

    for l in &mut lists{
        if sort.is_sorted(l){
            part_1_total += l[l.len()/2];
        } else {
            sort.sort(l);
            part_2_total += l[l.len()/2];
        }
    }

    println!("Day 5 Part 1: {}\nDay 5 Part 2: {}", part_1_total, part_2_total);
    Ok(())
}
