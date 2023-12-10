use std::fmt::Debug;
use std::str::FromStr;

/// Parse a whitespace-delimited string into a vector of objects of type T.
pub fn parse_on_whitespace<T>(s: &str) -> Result<Vec<T>, <T as FromStr>::Err> where T: FromStr {
    let mut v: Vec<T> = vec!();
    let mut parse_result: Result<T, <T as FromStr>::Err>;
    for sub in s.split_whitespace() {
        parse_result = sub.parse::<T>();
        match parse_result {
            Ok(value) => v.push(value),
            Err(e) => return Err(e)
        }
    }
    Ok(v)
}

/// Splits a string into two parts, one before the colon and one after.
/// Assumings the string is of a format (note the space after the colon):
///     <prefix>: <data>
/// Returns a tuple (prefix, data).
pub(crate) fn split_prefix(s: &str) -> (&str, &str) {
    let colon_i = s.find(':').expect("Could not find colon in string.");
    (&s[0..colon_i], &s[colon_i+2..])
}


#[derive(Clone, Debug)]
pub(crate) struct Grid<T> {
    vec: Vec<Vec<T>>
}

impl<T> Grid<T> {

    pub(crate) fn shape(&self) -> (usize, usize) {
        (self.vec.len(), self.vec[0].len())
    }

    pub(crate) fn is_valid(&self, posn: &(usize, usize)) -> bool {
        let (row, col) = posn;
        let (row_n, col_n) = self.shape();
        *row < row_n && *col < col_n
    }
    pub(crate) fn neighbors(&self, posn: &(usize, usize), incl_diag: bool)
        -> Vec<Option<(usize, usize)>> {
        let diffs: [i32; 3] = [-1, 0, 1];
        let mut n: Vec<Option<(usize, usize)>> = vec!();
        for r_diff in diffs.iter() {
            for c_diff in diffs.iter() {
                //println!("Testing ({r_diff}, {c_diff})");
                if *r_diff == 0 && *c_diff == 0 {
                    //println!("No diff; continuing");
                    continue
                }
                if (! incl_diag) && *r_diff != 0 && *c_diff != 0 {
                    //println!("Diagonal; continuing");
                    continue
                }
                n.push(self.apply_offset(posn, &(*r_diff, *c_diff)));
            }
        }
        n
    }

    /// Apply an offset to a position and return the resulting position, which is guaranteed to be
    /// within the grid.
    pub(crate) fn apply_offset(
        &self,
        posn: &(usize, usize),
        offset: &(i32, i32)
    ) -> Option<(usize, usize)> {
        let (row, col) = posn;
        let row_32 = *row as i32;
        let col_32 = *col as i32;
        let (n_rows, n_cols) = self.shape();
        let (row_off, col_off) = offset;
        let new_row_32 = row_32 + row_off;
        let new_col_32 = col_32 + col_off;
        if new_row_32 >= 0
            && new_row_32 < (n_rows as i32)
            && new_col_32 >= 0
            && new_col_32 < (n_cols as i32) {
            Some((new_row_32 as usize, new_col_32 as usize))
        } else {
            None
        }
    }

    pub(crate) fn iter_positions(&self) -> GridPositionIterator<T> {
        GridPositionIterator {
            grid: self,
            current_row: 0,
            current_col: 0
        }
    }
}

impl<T> Grid<T> where T: Copy + Eq {

    pub(crate) fn get(&self, posn: &(usize, usize)) -> Option<T> {
        let (row, col) = *posn;
        let (row_n, col_n) = self.shape();
        if row >= row_n || col >= col_n {
            None
        } else {
            Some(self.vec[row][col])
        }
    }

    pub(crate) fn find(&self, t: &T) -> Option<(usize, usize)> {
        for (row_i, row) in self.vec.iter().enumerate() {
            for (col_i, col) in row.iter().enumerate() {
                if col == t {
                    return Some((row_i, col_i))
                }
            }
        }
        None
    }

    pub(crate) fn iter_items(&self) -> GridItemIterator<T> {
        GridItemIterator {
            grid: self,
            current_row: 0,
            current_col: 0
        }
    }
}

impl FromStr for Grid<char> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec: Vec<Vec<char>> = vec!();
        for line in s.lines() {
            let v: Vec<char> = line.chars().collect();
            vec.push(v);
        }
        Ok(Grid { vec })
    }
}

pub(crate) struct GridItemIterator<'a, T> {
    grid: &'a Grid<T>,
    current_row: usize,
    current_col: usize
}

impl<'a, T> Iterator for GridItemIterator<'a, T> where T: Eq + Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let p = (self.current_row, self.current_col);
        let (_, n_col) = self.grid.shape();
        if self.current_col == n_col {
            self.current_col = 0;
            self.current_row += 1;
        } else {
            self.current_col += 1;
        }
        self.grid.get(&p)
    }
}

pub(crate) struct GridPositionIterator<'a, T> {
    grid: &'a Grid<T>,
    current_row: usize,
    current_col: usize
}

impl<'a, T> Iterator for GridPositionIterator<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let p = (self.current_row, self.current_col);
        if ! self.grid.is_valid(&p) {
            return None
        }
        let (_, n_col) = self.grid.shape();
        if self.current_col == n_col - 1 {
            self.current_col = 0;
            self.current_row += 1;
        } else {
            self.current_col += 1;
        }
        Some(p)
    }
}

