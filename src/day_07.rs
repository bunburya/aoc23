use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct Hand {
    card_values: [u32; 5],
    type_value: u32
}

impl Hand {
    fn from_str(s: &str, joker_rule: bool) -> Result<Self, String> {
        let j_val = if joker_rule { 1 } else { 11 };
        let mut card_values = [0; 5];
        let mut card_counts: HashMap<char, i32> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_numeric() {
                card_values[i] = match c.to_digit(10) {
                    Some(value) => value,
                    None => return Err(format!("Couldn't convert character '{c}' to number."))
                }
            } else {
                card_values[i] = match c {
                    'T' => 10,
                    'J' => j_val,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => return Err(format!("Unexpected card value {c}.")),
                }
            }
            let count = card_counts.entry(c).or_insert(0);
            *count += 1;
        }

        let j_count = if joker_rule {
            let c = *card_counts.entry('J').or_default();
            if c < 5 {
                // Don't remove if all cards are J as then we would be left with an empty list.
                card_counts.remove(&'J');
            }
            Some(c)
        } else {
            None
        };

        let mut most_common = match card_counts.keys().max_by_key(|k| card_counts[*k]) {
            Some(value) => value,
            None => return Err(format!("Could not find most common card in hand {s}."))
        };

        if let Some(c) = j_count {
            let count = card_counts.entry(*most_common).or_default();
            *count += c;

            // Re-calculate now that we have added the jokers
            most_common = match card_counts.keys().max_by_key(|k| card_counts[*k]) {
                Some(value) => value,
                None => return Err(format!("Could not find most common card in hand {s}."))
            };
        };

        let mut highest_count = card_counts[most_common];

        let type_value = match card_counts.len() {
            5 => 0,  // High card
            4 => 1,  // One pair
            3 => {
                match highest_count {
                    2 => 2,  // Two pair
                    3 => 3,  // Three of a kind
                    other => return Err(
                        format!("Length of card_counts is 3 but highest count is {other}.")
                    )
                }
            },
            2 => {
                match highest_count {
                    3 => 4,  // Full house
                    4 => 5,  // Four of a kind
                    other => return Err(
                        format!("Length of card_counts is 2 but highest count is {other}.")
                    )
                }
            },
            1 => 6, // Five of a kind
            other => return Err(format!("Length of card_counts is {other}."))
        };
        Ok(Hand { card_values, type_value })
    }
}

fn process_hands(s: &str, joker_rule: bool) -> String {
    let mut hands: [Vec<(Hand, i32)>; 7] = Default::default();
    for line in s.lines() {
        let mut split = line.split_whitespace();
        let hand = Hand::from_str(split.next().expect("Couldn't find hand."), joker_rule)
            .expect("Couldn't create Hand object from string.");
        let bid = split.next().expect("Couldn't find bid.")
            .parse::<i32>().expect("Couldn't parse bid.");
        hands[hand.type_value as usize].push((hand, bid));
    }
    let mut lower_ranked = 0;
    let mut total = 0;
    for i in 0..hands.len() {
        hands[i].sort_by_key(|t| t.0.card_values);
        for j in 0..hands[i].len() {
            let (_, bid) = hands[i][j];
            let rank = lower_ranked + j + 1;
            total += bid * (rank as i32);
        }
        lower_ranked += hands[i].len();
    }
    total.to_string()
}

pub(crate) fn part_1(s: &str) -> String {
    process_hands(s, false)
}

pub(crate) fn part_2(s: &str) -> String {
    process_hands(s, true)
}