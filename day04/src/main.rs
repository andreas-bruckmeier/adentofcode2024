use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_char_grid(filename: impl AsRef<Path>) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line"))
        .collect();
    let max_line_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let mut two_d_array: Vec<Vec<char>> = vec![vec![' '; max_line_length]; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        two_d_array[i].splice(..chars.len(), chars);
    }
    two_d_array
}

fn build_paths(grid: Vec<Vec<char>>) -> Vec<String> {

    let width = grid.len();
    let height = grid[0].len();

    let mut paths: Vec<String> = Vec::new();

    // rows
    for row in &grid {
        paths.push(row.iter().collect());
        paths.push(row.iter().rev().collect());
    }

    // cols
    for col in 0..width {
        let mut path: Vec<char> = Vec::new();
        for item in grid.iter().take(height) {
            path.push(item[col]);
        }
        paths.push(path.iter().rev().collect());
        paths.push(path.iter().collect());
    }

    // diagonal top left -> bottom right & reverse
    for diag in 0..(width+height-1) {
        let mut path: Vec<char> = Vec::new();
        for (row, item) in grid.iter().enumerate().take(width) {
            if diag >= row {
                let col = diag - row;
                if col < height {
                    path.push(item[col]);
                }
            }
        }
        paths.push(path.iter().rev().collect());
        paths.push(path.iter().collect());
        
    }

    // diagonal top right -> bottom left & reverse
    let width_signed = width as isize;
    let heigth_signed = height as isize;
    for diag in (-width_signed+1)..(heigth_signed) {
        let mut path: Vec<char> = Vec::new();
        for (row, item) in grid.iter().enumerate().take(width) {
            let col = row as isize + diag;
            if col >= 0 && col < heigth_signed {
                path.push(item[col as usize]);
            }

        }
        paths.push(path.iter().rev().collect());
        paths.push(path.iter().collect());
    }

    paths
}

fn main() {
    let grid = read_char_grid("input.txt");
    let paths = build_paths(grid);

    let part1: usize = paths.iter().map(|p| p.match_indices("XMAS").count()).sum();
    println!("part1: {}", part1);
}
