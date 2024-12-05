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
        UpdateStatus::Correct(update[update.len().saturating_div(2)].parse().unwrap())
    } else {
        UpdateStatus::Incorrect(
            sorted_update[sorted_update.len().saturating_div(2)]
                .parse()
                .unwrap(),
        )
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: Vec<_> = rules.lines().map(|l| l.split_once("|").unwrap()).collect();
    let updates: Vec<_> = updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .collect();

    let result1 = updates
        .iter()
        .map(|u| match check_update(u, &rules) {
            UpdateStatus::Correct(v) => v,
            UpdateStatus::Incorrect(_) => 0,
        })
        .sum::<u32>();
    let result2 = updates
        .iter()
        .map(|u| match check_update(u, &rules) {
            UpdateStatus::Correct(_) => 0,
            UpdateStatus::Incorrect(v) => v,
        })
        .sum::<u32>();
    println!("part1: {}", result1);
    println!("part2: {}", result2);
}
