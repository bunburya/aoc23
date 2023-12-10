use std::collections::HashSet;
use std::str::FromStr;
use geo::{Contains, coord, Polygon};
use geo::geometry::LineString;
use crate::common::Grid;

static N: (i32, i32) = (-1, 0);
static W: (i32, i32) = (0, -1);
static E: (i32, i32) = (0, 1);
static S: (i32, i32) = (1, 0);


/// Get the tiles that the pipe at the given position connect to. Doesn't verify that the locations
/// are valid.
fn get_connections(
    grid: &Grid<char>,
    posn: &(usize, usize),
    c: &char
)-> Option<[Option<(usize, usize)>; 2]> {
    let directions = match c {
        '|' => Some((N, S)),
        '-' => Some((E, W)),
        'L' => Some((N, E)),
        'J' => Some((N, W)),
        '7' => Some((S, W)),
        'F' => Some((S, E)),
        '.' => None,
        _ => panic!("Invalid pipe: {c}")
    };
    directions.map(|d| [grid.apply_offset(&posn, &d.0), grid.apply_offset(&posn, &d.1)])
}

fn get_next_posn(grid: &Grid<char>, from: &(usize, usize), posn: &(usize, usize), c: &char) -> Option<(usize, usize)> {
    let conns = get_connections(&grid, posn, c);
    match conns {
        Some(c) => {
            let c1 = c[0];
            let c2 = c[1];
            if c1 == Some(*from) {
                c2
            } else if c2 == Some(*from) {
                c1
            } else {
                None
            }
        },
        None => None
    }
}

fn find_path(grid: &Grid<char>) -> (i32, Vec<(usize, usize)>) {
    let s_posn = grid.find(&'S').expect("Could not find 'S' in grid.");
    let mut current = grid.neighbors(&s_posn, false);
    let mut prev: Vec<Option<(usize, usize)>> = vec!();
    let mut next: Vec<Option<(usize, usize)>> = vec!();
    let mut history: Vec<Vec<(usize, usize)>> = vec!();
    for _ in 0..current.len() {
        prev.push(Some(s_posn));
        next.push(None);
        history.push(vec!(s_posn));
    }
    let mut steps = 1;
    loop {
        for (i, c_posn) in current.iter().enumerate() {
            match c_posn {
                Some(p) => {
                    history[i].push(*p);
                    let c = grid.get(p)
                        .unwrap_or_else( | | panic!("Could not find character at {p:?}"));
                    let n_posn = get_next_posn(&grid, &prev[i].unwrap(), p, &c);
                    //println!("n_posn is {n_posn:?}");
                    if let Some(some_n_posn) = n_posn {
                        let other_path_i = next.iter().position(|p| p == &n_posn);
                        if let Some(j) = other_path_i {
                            let mut full_path = history[i].clone();
                            full_path.push(some_n_posn);
                            let other_path = &history[j];
                            for p in other_path.iter().rev() {
                                full_path.push(*p);
                            }
                            full_path.push(s_posn);
                            return (steps + 1, full_path.clone())

                        }
                    }
                    next[i] = n_posn;
                },
                None => next[i] = None
            }
        }
        steps += 1;
        prev = current.clone();
        current = next.clone();
    }
}
pub(crate) fn part_1(s: &str) -> String {
    let grid = Grid::from_str(s).expect("Could not create Grid.");
    let (steps, _) = find_path(&grid);
    steps.to_string()
}

pub(crate) fn part_2(s: &str) -> String {
    let grid = Grid::from_str(s).expect("Could not create Grid.");
    let (_, path) = find_path(&grid);
    let path_set: HashSet<&(usize, usize)> = HashSet::from_iter(&path);
    let linestr: LineString<f32> = path.iter()
        .map(|p| coord! { x: p.1 as f32, y: p.0 as f32 })
        .collect();
    let poly: Polygon<f32> = Polygon::new(linestr, vec!());
    let mut in_loop = 0;
    for p in grid.iter_positions() {
        if path_set.contains(&p) {
        } else if poly.contains(&coord! {x: p.1 as f32, y: p.0 as f32}) {
            in_loop += 1;
        }
    }
    in_loop.to_string()
}