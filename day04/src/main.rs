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

fn count_xmas(grid: &Vec<Vec<char>>) -> usize {


    let width = grid.len();
    let height = grid[0].len();

    let mut paths: Vec<String> = Vec::new();

    // rows
    for row in grid {
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

    paths.iter().map(|p| p.match_indices("XMAS").count()).sum()
}

fn count_x_mas(grid: &[Vec<char>]) -> usize {

    let n = grid.len();
    let m = grid[0].len();

    let mut count: usize = 0;

    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'A' {
                let top_left = if row > 0 && col > 0 {
                    Some(grid[row - 1][col - 1])
                } else {
                    None
                };

                let top_right = if row > 0 && col + 1 < m {
                    Some(grid[row - 1][col + 1])
                } else {
                    None
                };

                let bottom_left = if row + 1 < n && col > 0 {
                    Some(grid[row + 1][col - 1])
                } else {
                    None
                };

                let bottom_right = if row + 1 < n && col + 1 < m {
                    Some(grid[row + 1][col + 1])
                } else {
                    None
                };

                let mut found1 = false;
                let mut found2 = false;

                if let (Some(tl), Some(br)) = (top_left, bottom_right) {
                    if (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M') {
                        found1 = true;
                    }
                }

                if let (Some(tr), Some(bl)) = (top_right, bottom_left) {
                    if (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M') {
                        found2 = true;
                    }
                }

                if found1 && found2 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let grid = read_char_grid("input.txt");
    println!("part1: {}", count_xmas(&grid));
    println!("part2: {}", count_x_mas(&grid));
}
