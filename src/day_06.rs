use crate::common::{parse_on_whitespace, split_prefix};

fn count_winning_moves(t: i64, d: i64) -> i64 {
    // "-b formula" for quadratic equations
    // a = -1, b = t, c = -d
    let tf = t as f64;
    let df = d as f64;
    let r1: f64 = (-tf + (tf.powi(2) - (4.0 * (df + 0.5))).sqrt()) / -2.0;
    let r2: f64 = (-tf - (tf.powi(2) - (4.0 * (df + 0.5))).sqrt()) / -2.0;
    let diff: f64;
    if r1 > r2 {
        diff = r1.ceil() - r2;
    } else {
        diff = r2.ceil() - r1;
    }
    return diff.abs() as i64;
}

pub(crate) fn part_1(s: &str) -> String {
    let mut lines = s.lines();
    let t_line = lines.next().expect("Couldn't find time line.");
    let d_line = lines.next().expect("Couldn't find distance line.");

    let (_, time_s) = split_prefix(&t_line);
    let (_, dist_s) = split_prefix(&d_line);

    let times = parse_on_whitespace::<i64>(&time_s)
        .expect("Could not parse times.");
    let distances = parse_on_whitespace::<i64>(&dist_s)
        .expect("Could not parse distances.");

    let mut product = 1;

    for (i, t) in times.iter().enumerate() {
        let n = count_winning_moves(*t, distances[i]);
        product *= n;
    }
    product.to_string()
}

pub(crate) fn part_2(s: &str) -> String {
    let mut lines = s.lines();
    let t = lines.next().expect("Could not find time line.")
        .chars().filter(|c| c.is_numeric())
        .collect::<String>().parse::<i64>().expect("Could not parse time to string.");
    let d = lines.next().expect("Could not find distance line.")
        .chars().filter(|c| c.is_numeric())
        .collect::<String>().parse::<i64>().expect("Could not parse distance to string.");

    count_winning_moves(t, d).to_string()
}