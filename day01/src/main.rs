use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn part1(list1: &Vec<u32>, list2: &Vec<u32>) -> u32 {
    zip(list1, list2).map(|i| i.0.abs_diff(*i.1)).sum()
}

fn part2(list1: &Vec<u32>, list2: &Vec<u32>) -> u32 {
    let mut m: HashMap<u32, u32> = HashMap::new();
    for x in list2 {
        *m.entry(*x).or_default() += 1;
    }
    let mut result: u32 = 0;
    for number in list1 {
        let increment = number.saturating_mul(m.get(number).cloned().unwrap_or_default());
        result = result.saturating_add(increment);
    }
    result
}

fn main() {
    let lines: Vec<_> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in lines {
        let numbers = line
            .split_once(' ')
            .map(|l| (l.0.parse().unwrap(), l.1.trim().parse().unwrap()))
            .unwrap();
        list1.push(numbers.0);
        list2.push(numbers.1);
    }

    list1.sort();
    list2.sort();

    println!(
        "part1: {}, part2: {}",
        part1(&list1, &list2),
        part2(&list1, &list2)
    );
}
