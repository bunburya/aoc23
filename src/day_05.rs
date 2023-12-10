//use std::cmp::{max, min};
use std::str::FromStr;
use std::string::ParseError;
use crate::common::{parse_on_whitespace, split_prefix};


#[derive(Copy, Clone, Debug)]
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

impl RangeMap {
    fn get(&self, k: i64) -> Option<i64> {
        if k < self.src_start || k >= self.src_start + self.range_len {
            return None
        }
        Some(self.dst_start + (k - self.src_start))
    }

/*
    /// Return three optional sub-ranges of the given range: one range of values below this range,
    /// one range of values within this range and one range of values above this range.
    fn get_range(&self, start: i64, end: i64) -> (
        Option<(i64, i64)>,
        Option<(i64, i64)>,
        Option<(i64, i64)>
    ) {
        let src_end = self.src_start + self.range_len;
        let mut below: Option<(i64, i64)> = None;
        let mut matches: Option<(i64, i64)> = None;
        let mut above: Option<(i64, i64)> = None;
        if start < self.src_start {
            below = Some((start, min(end, self.src_start-1)));
        }
        if end > src_end {
            above = Some((max(start, src_end), end));
        }
        if start > self.src_start || end < src_end {
            matches = Some((max(start, self.src_start), min(end, src_end)));
        }
        (below, matches, above)
    }

    /// Return how much would need to be added to a given source value to map it to a destination
    /// value.
    fn get_increment(&self) -> i64 {
        self.dst_start - self.src_start
    }

    /// The end of the source range represented by this range map
    fn src_end(&self) -> i64 {
        self.src_start + self.range_len - 1
    }


 */

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
    let seeds: Vec<i64> = parse_on_whitespace(seed_str).expect("Could not parse seeds from seed line.");
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
    let seed_ranges_desc: Vec<i64> = parse_on_whitespace(seed_str)
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
            let numbers = parse_on_whitespace::<i64>(line)
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
/*
fn incr_range(range: &(i64, i64), n: i64) -> (i64, i64) {
    (range.0 + n, range.1 + n)
}

pub(crate) fn part_2_fast(s: &str) -> String {
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
            current_map_vec.push(RangeMap::from_str(line).unwrap())
        }
    }
    range_maps.push(current_map_vec.clone());

    let mut ranges: Vec<(i64, i64)> = seed_ranges;
    for map_vec in range_maps {
        println!("Reached next round");
        let mut sorted_map_vec = map_vec.clone();
        sorted_map_vec.sort_by_key(|mr| mr.src_start);
        ranges.sort_by_key(|r| r.0);
        //println!("{:?}", ranges);
        let mut i = 0;
        let mut leftover: Option<(i64, i64)> = None;
        let mut next_ranges: Vec<(i64, i64)> = vec!();

        loop {
            //println!("{i}");
            let mut matched = false;
            let (start, end) = match leftover {
                // If we have a leftover value, use it and don't increment i
                Some(range) => range,
                None => {
                    if i > ranges.len() - 1 {
                        break;
                    }
                    let range = ranges[i];
                    i += 1;
                    range
                }
            };
            //println!("Start: {leftover:?}");
            leftover = None;
            for range_map in &sorted_map_vec {
                //println!("{:?}", range_map);
                //println!("{}, {}", range_map.src_end(), start);
                println!("Comparing range map {range_map:?} against range ({start}, {end}).");
                if range_map.src_end() <= start {
                    println!("Range map below start of range; continuing.");
                    continue
                }
                if range_map.src_start > end {
                    println!("Range map above end of range; breaking.");
                    break
                }
                let (below, matches, above) = range_map.get_range(start, end);
                if let Some(below_range) = below {
                    println!("Found {below_range:?} below range map. Adding.");
                    next_ranges.push(below_range);
                    matched = true;
                }
                if let Some(matches_range) = matches {
                    let to_add = incr_range(&matches_range, range_map.get_increment());
                    println!("Found {matches_range:?} in range map. Adding {to_add:?}");
                    next_ranges.push(to_add);
                    matched = true;
                }
                if ! above.is_none() {
                    let above_range = above.unwrap();
                    println!("Found {above_range:?} above range. Testing this next.");
                    leftover = above;
                    matched = true;
                }
                //println!("End: {leftover:?}");
            }
            if ! matched {
                // We didn't match any range so push the whole range through
                println!("Didn't match ({start}, {end}). Pushing through.");
                next_ranges.push((start, end))
            }
        }
        next_ranges.sort_by_key(|r| r.0);
        ranges = next_ranges;
    }
    //println!("{:?}", ranges);
    ranges[0].0.to_string()
}

 */