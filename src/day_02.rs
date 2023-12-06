use std::collections::HashMap;

fn partition_game_str(line: &str) -> (&str, &str) {
    let colon_i = line.find(":").expect("No colon found in line.");
    let game_id_str = &line[5..colon_i];
    let rounds_str = &line[colon_i+2..];
    (game_id_str, rounds_str)
}

fn get_samples(round_str: &str) -> Vec<(&str, i32)> {
    let mut all_samples: Vec<(&str, i32)> = vec!();
    let rounds = round_str.split("; ");
    for r in rounds {
        let samples = r.split(", ");
        for s in samples {
            let (number_s, color) = s.split_once(" ")
                .expect("Text should be in format `<number> <color>");
            let number = number_s.parse::<i32>()
                .expect("Could not parse number from text");
            all_samples.push((color, number))
        }
    }
    all_samples
}

fn is_possible(rounds_str: &str, bag: &HashMap<&str, i32>) -> bool {
    for (color, number) in get_samples(rounds_str) {
        if number > bag[color] {
            return false
        }
    }
    true
}

fn minimum_power(rounds_str: &str) -> i32 {
    let mut max: HashMap<&str, i32> = HashMap::new();
    for (color, number) in get_samples(rounds_str) {
        let current_min = match max.get(color) {
            Some(val) => *val,
            None => number - 1
        };
        if number > current_min {
            max.insert(color, number);
        }

    }
    max.values().product::<i32>()
}

pub(crate) fn part_1(s: &str) -> String {
    let mut bag = HashMap::new();
    bag.insert("red", 12);
    bag.insert("green", 13);
    bag.insert("blue", 14);

    let mut total = 0;

    for line in s.lines() {
        let (game_id_str, rounds_str) = partition_game_str(line);
        let game_id = game_id_str.parse::<i32>()
            .expect("Could not parse game ID from text");
        if is_possible(rounds_str, &bag) {
            total += game_id;
        }
    }
    total.to_string()
}

pub(crate) fn part_2(s: &str) -> String {
    let mut total = 0;
    for line in s.lines() {
        let (_, rounds_str) = partition_game_str(line);
        total += minimum_power(rounds_str);
    }
    total.to_string()
}