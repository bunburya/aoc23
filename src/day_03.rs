use std::cmp;

/// Iterate over all tiles that are adjacent to any tile between (row, start_col) and (row, end_col)
/// inclusive.
struct Adjacent {
    row: usize,
    start_col: usize,
    end_col: usize,
    all_rows: usize,
    all_cols: usize,
    i_row: usize,
    i_col: usize
}

impl Adjacent {
    fn new(
        row: usize,
        start_col: usize,
        end_col: usize,
        all_rows: usize,
        all_cols: usize
    ) -> Adjacent {
        let i_row: usize = row.saturating_sub(1);
        let i_col: usize = start_col.saturating_sub(1);
        Adjacent {
            row,
            start_col,
            end_col,
            all_rows,
            all_cols,
            i_row,
            i_col
        }
    }
}
impl Iterator for Adjacent {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let max_row = cmp::min(self.row + 1, self.all_rows - 1);
        let max_col = cmp::min(self.end_col + 1, self.all_cols - 1);
        if self.i_row > max_row {
            return None
        }
        let to_return = (self.i_row, self.i_col);

        if self.i_col >= max_col {
            // Have reached the end of a row.  We have already checked that we are not at the last
            // row so move to the start of the next row.
            self.i_row += 1;
            self.i_col = self.start_col.saturating_sub(1);
        } else {
            // Not at the end of a row so move on to the next column
            if (self.i_row == self.row)
                && (self.i_col >= self.start_col && self.i_col <= self.end_col) {
                // We are inside the number string being checked so fast forward to the end
                self.i_col = cmp::min(self.end_col + 1, max_col);
            } else {
                self.i_col += 1;
            }
        }
        Some(to_return)
    }
}

/// Create a 2-d vector from a newline-delimited input string and return a tuple containing the
/// vector, the number of rows and the number of columns.
fn get_grid(s: &str) -> (Vec<Vec<char>>, usize, usize) {
    let mut v: Vec<Vec<char>> = vec!();
    for line in s.lines() {
        v.push(line.chars().collect());
    }
    let n_rows = v.len();
    let n_cols = v[0].len();
    (v, n_rows, n_cols)
}

fn is_symbol(c: char) -> bool {
    ! (c.is_numeric() || c == '.')
}

/// Check if any of the tiles adjacent to (row, col) contain symbols.
fn has_adjacent_symbol(
    grid: &Vec<Vec<char>>,
    row: usize,
    start_col: usize,
    end_col: usize,
    all_rows: usize,
    all_cols: usize
) -> bool {
    for (r, c) in Adjacent::new(row, start_col, end_col, all_rows, all_cols) {
        if is_symbol(grid[r][c]) {
            return true
        }
    }
    false
}

/// Get the entire number that the digit at (row, col) is a part of. Return a tuple containing
/// the number and the start and end columns of the number string.
fn get_number(
    grid: &Vec<Vec<char>>, row: usize, col: usize, all_cols: usize
) -> (i32, usize, usize) {
    let mut c = grid[row][col];
    let mut s = String::from(c);
    let mut start_col = col;
    let mut end_col = col;
    // first go backwards
    for i in (0..col).rev() {
        c = grid[row][i];
        if ! c.is_numeric() {
            break
        }
        s.push(c);
        start_col = i;
    }
    // now reverse the string and start looking forward
    s = s.chars().rev().collect::<String>();
    for i in (col+1)..all_cols {
        c = grid[row][i];
        if ! c.is_numeric() {
            break
        }
        s.push(c);
        end_col = i;
    }
    let num = s.parse::<i32>().unwrap();
    (num, start_col, end_col)
}

/// Return the gear ratio for position, or None if the character at the position is not a gear.
fn get_gear_ratio(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    all_rows: usize,
    all_cols: usize
) -> Option<i32> {
    if grid[row][col] != '*' {
        return None
    }
    let mut chr: char;
    let mut num: i32;
    let mut start_col: usize;
    let mut end_col: usize;
    // A vector containing (number, row, start_col, end_col) for each adjacent number string
    let mut adj_nums: Vec<(i32, usize, usize, usize)> = vec!();

    'outer: for (r, c) in Adjacent::new(row, col, col, all_rows, all_cols) {
        for (_, row, s_col, e_col) in &adj_nums {
            if r == *row && c >= *s_col && c <= *e_col {
                // We've already found this number
                continue 'outer
            }
        }
        chr = grid[r][c];
        if chr.is_numeric() {
            if adj_nums.len() == 2 {
                // We already identified two numbers and now we've identified a third. We therefore
                // know this isn't a gear so immediately return None
                return None
            }
            (num, start_col, end_col) = get_number(grid, r, c, all_cols);
            adj_nums.push((num, r, start_col, end_col));
        }
    }
    if adj_nums.len() == 2 {
        return Some(adj_nums[0].0 * adj_nums[1].0)
    }
    None
}

pub(crate) fn part_1(s: &str) -> String {
    let (grid, all_rows, all_cols) = get_grid(s);
    let mut num_s: String;
    let mut num_start_col: Option<usize>;
    let mut total = 0;

    for (ir, row) in grid.iter().enumerate() {
        num_start_col = None;
        num_s = String::new();
        for (ic, c) in row.iter().enumerate() {
            if c.is_numeric() {
                if num_s.is_empty() {
                    // start of a number
                    num_start_col = Some(ic);
                }
                num_s.push(*c);
            } else {
                if ! num_s.is_empty() {
                    // We have encountered a non-digit after a string of digits, ie, we have a full
                    // number
                    if has_adjacent_symbol(
                        &grid, ir, num_start_col.unwrap(), ic.saturating_sub(1), all_rows, all_cols
                    ) {
                        total += num_s.parse::<i32>().unwrap();
                    }
                    num_start_col = None;
                    num_s = String::new();
                }
            }
        }
        if ! num_s.is_empty() {
            // Check number string again at the end of the row

            if has_adjacent_symbol(
                &grid, ir, num_start_col.unwrap(), all_cols-1, all_rows, all_cols
            ) {

                total += num_s.parse::<i32>().unwrap();

            }
        }

    }
    total.to_string()
}

pub(crate) fn part_2(s: &str) -> String {
    let (grid, all_rows, all_cols) = get_grid(s);
    let mut ratio: Option<i32>;
    let mut total = 0;
    for (ir, row) in grid.iter().enumerate() {
        for (ic, chr) in row.iter().enumerate() {
            if *chr != '*' {
                continue
            }
            ratio = get_gear_ratio(&grid, ir, ic, all_rows, all_cols);
            match ratio {
                Some(number) => total += number,
                None => continue
            }
        }
    }
    total.to_string()
}