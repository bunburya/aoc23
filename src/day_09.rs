use std::ops::{Sub, Add};
use std::process::Output;

use crate::common::parse_str;

fn all_same<T>(seq: &Vec<T>) -> bool where T: Ord {
    for i in 1..seq.len() {
        if seq[i] != seq[i-1] {
            return false
        }
    }
    true
}

fn get_diffs<T>(seq: &Vec<T>) -> Vec<T> where T: Sub<Output=T> + Copy {
    let mut diffs: Vec<T> = vec!();
    for i in 1..seq.len() {
        diffs.push(seq[i] - seq[i-1]);
    }
    diffs
}


fn extrapolate<T>(seq: &Vec<T>) -> T where T: Sub<Output = T> + Add<Output = T> + Ord + Copy {
    let last = *seq.last().expect("Could not find last element of sequence.");
    if all_same(seq) {
        last
    } else {
        let diffs = get_diffs(seq);
        last + extrapolate(&diffs)
    }
}

pub(crate) fn part_1(s: &str) -> String {
    let mut total: i64 = 0;
    for line in s.lines() {
        total += extrapolate(&parse_str::<i64>(line).unwrap())

    }
    total.to_string()
}

pub(crate) fn part_2(s: &str) -> String {
    let mut total: i64 = 0;
    for line in s.lines() {
        let mut seq = parse_str::<i64>(line).unwrap();
        seq.reverse();
        total += extrapolate(&seq);
    }
    total.to_string()
}