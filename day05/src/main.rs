use std::cmp::Ordering;
use std::fs;

enum UpdateStatus {
    Correct(u32),
    Incorrect(u32),
}

fn check_update(update: &Vec<&str>, rules: &[(&str, &str)]) -> UpdateStatus {
    let mut sorted_update = update.clone();
    sorted_update.sort_by(|a, b| {
        if rules.iter().any(|rule| *rule == (*a, *b)) {
            return Ordering::Less;
        }
        if rules.iter().any(|rule| *rule == (*b, *a)) {
            return Ordering::Greater;
        }
        Ordering::Equal
    });

    if *update == sorted_update {
        UpdateStatus::Correct(update[update.len() / 2].parse().unwrap())
    } else {
        UpdateStatus::Incorrect(sorted_update[sorted_update.len() / 2].parse().unwrap())
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input not found");
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: Vec<_> = rules.lines().map(|l| l.split_once("|").unwrap()).collect();
    let (mut part1, mut part2): (u32, u32) = (0, 0);

    for update in updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .collect::<Vec<_>>()
    {
        match check_update(&update, &rules) {
            UpdateStatus::Correct(v) => part1 += v,
            UpdateStatus::Incorrect(v) => part2 += v,
        }
    }

    println!("part1: {}, part2: {}", part1, part2);
}
