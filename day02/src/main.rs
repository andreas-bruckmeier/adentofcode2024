use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
    Undefined,
}

fn is_valid_level_pair(pair: (&u32, &u32), direction: Direction) -> Option<Direction> {
    if !(1..=3).contains(&pair.0.abs_diff(*pair.1)) {
        return None;
    }
    if pair.1 > pair.0 && (direction == Direction::Undefined || direction == Direction::Increasing)
    {
        return Some(Direction::Increasing);
    }
    if pair.0 > pair.1 && (direction == Direction::Undefined || direction == Direction::Decreasing)
    {
        return Some(Direction::Decreasing);
    }
    None
}

fn is_valid_report(report: &[u32], drop: Option<usize>) -> bool {
    let mut direction = Direction::Undefined;
    report
        .iter()
        .enumerate()
        .filter(|&(i, _)| drop.is_none() || i != drop.unwrap()) // skip level at position
        .map(|(_, v)| v)
        .tuple_windows::<(_, _)>() // Create sliding window pairs
        .all(|pair| {
            match is_valid_level_pair(pair, direction) {
                Some(d) => {
                    direction = d;
                    true
                }
                None => false,
            }
        })
}

fn part1(reports: &[Vec<u32>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            if is_valid_report(&report, None) {
                return true;
            }
            false
        })
        .count()
}

fn part2(reports: &[Vec<u32>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            for (position, _) in report.iter().enumerate() {
                if is_valid_report(&report, Some(position)) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn main() {
    let reports: Vec<Vec<_>> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.split_whitespace().map(|d| d.parse().unwrap()).collect())
        .collect();
    println!("part1: {}", part1(&reports));
    println!("part2: {}", part2(&reports));
}
