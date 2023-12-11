use std::cmp::{max, min};
use std::collections::HashSet;
use std::str::FromStr;
use num::abs;
use crate::common::Grid;

/// Return a tuple of (empty rows, empty columns)
fn get_empties(grid: &Grid<char>) -> (HashSet<usize>, HashSet<usize>) {
    let mut empty_rows: HashSet<usize> = HashSet::new();
    let mut empty_cols: HashSet<usize> = HashSet::new();

    let (rows, cols) = grid.shape();
    for i in 0..rows {
        if grid.find_col(i, &'#').is_none() {
            empty_rows.insert(i);
        }
    }
    for i in 0..cols {
        if grid.find_row(i, &'#').is_none() {
            empty_cols.insert(i);
        }
    }
    (empty_rows, empty_cols)
}

fn get_galaxies(grid: &Grid<char>) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = vec!();
    for p in grid.iter_positions() {
        let c = grid.get(&p).unwrap();
        if c == '#' {
            positions.push(p);
        }
    }
    positions
}

fn get_pairs(positions: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    let mut pairs: Vec<((usize, usize), (usize, usize))> = vec!();
    for i in 0..positions.len() {
        for j in i+1..positions.len() {
            pairs.push((positions[i], positions[j]));
        }
    }
    pairs
}

fn get_distance(
    p1: &(usize, usize), p2: &(usize, usize),
    empty_rows: &HashSet<usize>, empty_cols: &HashSet<usize>,
    expansion_size: &i32
) -> i32 {
    let mut row_diff = abs(p1.0 as i32 - p2.0 as i32);
    for i in min(p1.0, p2.0)..max(p1.0, p2.0) {
        if empty_rows.contains(&i) {
            // subtract 1 to reflect that we are *replacing* the empty row with the new rows
            row_diff += expansion_size - 1;
        }
    }
    let mut col_diff = abs(p1.1 as i32 - p2.1 as i32);
    for i in min(p1.1, p2.1)..max(p1.1, p2.1) {
        if empty_cols.contains(&i) {
            col_diff += expansion_size - 1;
        }
    }
    row_diff + col_diff
}

fn solve(s: &str, expansion_size: &i32) -> String {
    let grid = Grid::from_str(s).expect("Could not create Grid.");
    let (empty_rows, empty_cols) = get_empties(&grid);
    let galaxies = get_galaxies(&grid);
    let pairs = get_pairs(&galaxies);
    let mut total: i64 = 0;
    for (p1, p2) in pairs {
        let d = get_distance(&p1, &p2, &empty_rows, &empty_cols, expansion_size);
        total += d as i64;
    }
    total.to_string()
}
pub(crate) fn part_1(s: &str) -> String {
    solve(s, &2)
}

pub(crate) fn part_2(s: &str) -> String {
    solve(s, &1000000)
}