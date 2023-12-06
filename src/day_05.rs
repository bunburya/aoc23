use std::str::FromStr;
use std::string::ParseError;
use crate::common::{parse_str, split_prefix};


struct RangeMap {
    dst_start: i64,
    src_start: i64,
    range_len: i64
}

impl FromStr for RangeMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let dst_start = tokens.next().expect("Could not find source start in str.")
            .parse().expect("Could not parse source start from str.");
        let src_start = tokens.next().expect("Could not find destination start in str.")
            .parse().expect("Could not parse destination start from str.");
        let range_len = tokens.next().expect("Could not find range length in str.")
            .parse().expect("Could not parse range length from str");
        Ok(RangeMap { dst_start, src_start, range_len })
    }
}

impl Clone for RangeMap {
    fn clone(&self) -> Self {
        RangeMap {
            dst_start: self.dst_start,
            src_start: self.src_start,
            range_len: self.range_len
        }
    }
}

impl RangeMap {
    fn get(&self, k: i64) -> Option<i64> {
        if k < self.src_start || k >= self.src_start + self.range_len {
            return None
        }
        Some(self.dst_start + (k - self.src_start))
    }
}

fn get_all(k: i64, maps: &Vec<RangeMap>) -> Option<i64> {
    for m in maps {
        if let Some(v) = m.get(k) {
            return Some(v)
        }
    }
    None
}

pub(crate) fn part_1(s: &str) -> String {

    let mut lines = s.lines();
    let seed_line = lines.next().expect("Could not find seed line.");
    let (_, seed_str) = split_prefix(seed_line);
    let seeds: Vec<i64> = parse_str(seed_str).expect("Could not parse seeds from seed line.");
    lines.next();  // Get first blank line out of the way
    lines.next();  // Get the first header out of the way
    let mut range_maps: Vec<Vec<RangeMap>> = vec!();
    let mut current_map_vec: Vec<RangeMap> = vec!();
    for line in lines {
        if line == "" {
            range_maps.push(current_map_vec.clone());
        } else if line.chars().next().unwrap().is_alphabetic() {
            // We found a "heading" indicating the start of the next list of mappings
            current_map_vec = vec!();
        } else {
            current_map_vec.push(RangeMap::from_str(line).unwrap())
        }
    }
    range_maps.push(current_map_vec.clone());
    let mut min_loc: Option<i64> = None;
    for seed in seeds {
        let mut k = seed;
        for v in &range_maps {
            k = get_all(k, v).unwrap_or(k);
        }
        if k < min_loc.unwrap_or(k+1) {
            min_loc = Some(k)
        }
    }
    min_loc.expect("Could not find lowest location.").to_string()
}

pub(crate) fn part_2(s: &str) -> String {

    let mut lines = s.lines();
    let seed_line = lines.next().expect("Could not find seed line.");
    let (_, seed_str) = split_prefix(seed_line);
    let seed_ranges_desc: Vec<i64> = parse_str(seed_str)
        .expect("Could not parse seeds from seed line.");
    let mut seed_ranges: Vec<(i64, i64)> = vec!();

    for i in (0..seed_ranges_desc.len()-1).step_by(2) {
        let start = seed_ranges_desc[i];
        let range = seed_ranges_desc[i+1];
        let end = start + range;
        seed_ranges.push((start, end));
    }

    lines.next();
    lines.next();
    let mut range_maps: Vec<Vec<RangeMap>> = vec!();
    let mut current_map_vec: Vec<RangeMap> = vec!();
    for line in lines {
        if line == "" {
            range_maps.push(current_map_vec.clone());
        } else if line.chars().next().unwrap().is_alphabetic() {
            // We found a "heading" indicating the start of the next list of mappings
            current_map_vec = vec!();
        } else {
            let numbers = parse_str::<i64>(line)
                .expect("Could not parse numbers from string.");
            current_map_vec.push(RangeMap {
                dst_start: numbers[1],
                src_start: numbers[0],
                range_len: numbers[2]
            })
        }
    }
    range_maps.push(current_map_vec.clone());
    range_maps.reverse();
    let mut loc = 0;
    loop {
        let mut k = loc;
        for v in &range_maps {
            k = get_all(k, &v).unwrap_or(k);
        }
        for (start, end) in &seed_ranges {
            if k > *start && k < *end {
                return loc.to_string()
            }
        }
        loc += 1;
    }
}