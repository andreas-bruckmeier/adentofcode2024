use std::fs;

fn update_ok(update: &Vec<&str>, rules: &Vec<(&str, &str)>) -> u32 {
    for (page_index, page) in update.iter().enumerate() {
        // rules for left side
        for rule in rules.iter().filter(|rule| rule.0 == *page) {
            let left = &update[0..page_index];
            if left.contains(&rule.1) {
                return 0;
            }
        }
        // rules for right side
        for rule in rules.iter().filter(|rule| rule.1 == *page) {
            let right = &update[page_index..];
            if right.contains(&rule.0) {
                return 0;
            }
        }
    }

    let foo = update.len().saturating_div(2);
    return update[foo].parse().unwrap();
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: Vec<_> = rules.lines().map(|l| l.split_once("|").unwrap()).collect();
    let updates: Vec<_> = updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .collect();

    let result = updates.iter().map(|u| update_ok(&u, &rules)).sum::<u32>();
    println!("part1: {}", result);
}
