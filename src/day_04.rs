use std::collections::{HashSet, VecDeque};

fn get_numbers(s: &str) -> (HashSet<i32>, HashSet<i32>) {
    let mut winners: HashSet<i32> = HashSet::new();
    let mut ours: HashSet<i32> = HashSet::new();

    let colon_i = s.find(":").expect("Could not find colon in line.");
    let nums_str = &s[colon_i+2..];
    let mut num_split = nums_str.split(" | ");
    for n in num_split.next().unwrap().split_whitespace() {
        winners.insert(n.parse::<i32>().expect("Could not parse to integer"));
    }
    for n in num_split.next().unwrap().split_whitespace() {
        ours.insert(n.parse::<i32>().expect("Could not parse to integer"));
    }
    (winners, ours)
}

fn get_match_count(s: &str) -> u32 {
    let (winners, ours) = get_numbers(s);
    winners.intersection(&ours).count() as u32
}


pub(crate) fn part_1(s: &str) -> String {
    let mut total = 0;
    for line in s.lines() {
        let our_winners = get_match_count(line);
        if our_winners > 0 {
            let score = 2u32.pow(our_winners - 1);
            total += score;
        }
    }
    total.to_string()
}

pub(crate) fn part_2(s: &str) -> String {
    let mut additional_copies: VecDeque<u32> = VecDeque::new();
    let mut num_cards = 0;

    for line in s.lines() {
        let copies = additional_copies.pop_front().unwrap_or(0);
        let mul = 1 + copies;
        let mut matches = get_match_count(line);
        num_cards += 1 + (matches * mul);
        for m in additional_copies.iter_mut() {
            if matches == 0 {
                break
            }
            matches -= 1;
            *m += mul;
        }
        while matches > 0 {
            additional_copies.push_back(mul);
            matches -= 1;
        }
    }
    num_cards.to_string()
}