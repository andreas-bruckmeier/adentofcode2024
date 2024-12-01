use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap());

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

    let iter = zip(list1, list2);

    let result: u32 = iter.map(|i| i.0.abs_diff(i.1)).sum();

    println!("{:#?}", result);
}
