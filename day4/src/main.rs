#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::fs;

#[derive(Clone)]
struct Data {
    is_paper_roll : bool,
    can_access : bool,
}

// part 1 - 1587
fn main() {
    let mut grid: Vec<Vec<Data>> = populate_grid();
    //print_grid(&grid);
    determine_accessibility(&mut grid);
    count_accessible_paper_rolls(&grid);
}

fn populate_grid() -> Vec<Vec<Data>> {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let numRow = contents.lines().count();
    let numCol = contents.lines().next().unwrap().len();

    let mut grid: Vec<Vec<Data>> = vec![vec![Data { is_paper_roll: false, can_access: false }; numCol]; numRow];
    for (i, line) in contents.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '@' => grid[i][j].is_paper_roll = true,
                _ => (),
            }
        }
    }

    return grid;
}

fn determine_accessibility(grid: &mut Vec<Vec<Data>>) {
    let numRow = grid.len();
    let numCol = grid[0].len();

    for i in 0..numRow {
        for j in 0..numCol {
            if grid[i][j].is_paper_roll {
                if count_paper_rolls(i, j, grid) < 4 {
                    grid[i][j].can_access = true;
                }
            } else {
                grid[i][j].can_access = false;
            }
        }
    }
}

fn count_paper_rolls(row: usize, col: usize, grid: &Vec<Vec<Data>>) -> i32 {
    let directions = [(-1, -1), (-1, 0), (-1, 1 ),
                                           (0, -1), (0, 1),
                                           (1, -1), (1, 0), (1, 1)];
    let mut count = 0;
    let numRow = grid.len() as isize;
    let numCol = grid[0].len() as isize;

    for (rowDirection, columnDirection) in directions.iter() {
        let adjacentRow   = row as isize + rowDirection;
        let adjacentCol = col as isize + columnDirection;

        if adjacentRow >= 0 && adjacentRow < numRow && adjacentCol >= 0 && adjacentCol < numCol {
            if grid[adjacentRow as usize][adjacentCol as usize].is_paper_roll {
                count += 1;
            }
        }
    }

    return count;
}

fn count_accessible_paper_rolls(grid: &Vec<Vec<Data>>) {
    let mut accessible_count = 0;

    for row in grid {
        for data in row {
            if data.can_access {
                accessible_count += 1;
            }
        }
    }

    println!("Number of accessible paper rolls: {}", accessible_count);
}

fn print_grid(grid: &Vec<Vec<Data>>) {
    for row in grid {
        for cell in row {
            if cell.is_paper_roll {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
