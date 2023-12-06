use std::collections::HashMap;

fn new_digit_word_map<'a>() -> HashMap<&'a str, char> {
    let mut digits = HashMap::new();
    digits.insert("one", '1');
    digits.insert("two", '2');
    digits.insert("three",'3');
    digits.insert("four", '4');
    digits.insert("five", '5');
    digits.insert("six", '6');
    digits.insert("seven", '7');
    digits.insert("eight", '8');
    digits.insert("nine", '9');
    digits
}

fn words_to_digits(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let digit_word_map = new_digit_word_map();
    let mut new_s = String::new();
    let mut next_push_unmatched = 0;  // the next position at (or after) which we will push a
                                      // character, which doesn't match a "number word", to the new
                                      // string.
    let mut word_size: usize;  // the size of a number word
    let mut matched: bool;  // whether a number word match has been found
    let mut to_test: String;  // the slice of text we are to test against each number word
    for (i, c) in chars.iter().enumerate() {
        matched = false;
        for (w, d) in digit_word_map.iter() {
            // for each number word, get the length of that word, look ahead that many
            // characters and compare to the number word
            word_size = w.len();
            if i + word_size > chars.len() {
                // looking ahead would take us out of bounds (so word can't match)
                continue;
            }
            to_test = chars[i..i+word_size].iter().collect();
            if to_test == *w {
                // found a match
                new_s.push(*d);
                matched = true;
                next_push_unmatched = i + word_size;
                break;
            }
        }
        if (!matched) && (i >= next_push_unmatched) {
            // didn't match any number word, so just push the character directly
            new_s.push(*c);
        }
    }
    new_s
}

fn first_digit<I>(chars: I) -> char where I: Iterator<Item = char> {
    // Find the first numeric character in a sequence of characters
    for c in chars {
        if c.is_numeric() {
            return c
        }
    }
    panic!("No digits found.")
}

pub(crate) fn part_1(s: &str) -> String {
    let mut cal: Vec<u32> = vec!();
    for line in s.lines() {
        let chars = line.chars();
        let first = first_digit(chars.clone()).to_digit(10)
            .expect("First digit character cannot be converted to number.");
        let last = first_digit(chars.rev()).to_digit(10)
            .expect("Last digit character cannot be converted to number.");
        cal.push((first * 10) + last)
    }
    let total: u32 = cal.iter().sum();
    total.to_string()
}

pub(crate) fn part_2(s: &str) -> String {
    let fixed = words_to_digits(s);
    part_1(&fixed)
}