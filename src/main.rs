mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod common;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

use std::{fs, path};
use std::env;
use std::time::Instant;

fn main() {

    let funcs: [[Option<fn(&str) -> String>; 2]; 25] = [
        [Some(day_01::part_1), Some(day_01::part_2)],
        [Some(day_02::part_1), Some(day_02::part_2)],
        [Some(day_03::part_1), Some(day_03::part_2)],
        [Some(day_04::part_1), Some(day_04::part_2)],
        [Some(day_05::part_1), Some(day_05::part_2)],
        [Some(day_06::part_1), Some(day_06::part_2)],
        [Some(day_07::part_1), Some(day_07::part_2)],
        [Some(day_08::part_1), Some(day_08::part_2)],
        [Some(day_09::part_1), Some(day_09::part_2)],
        [None, None],
        [None, None],
        [None, None],
        [None, None],
        [None, None],
        [None, None],
        [None, None],
        [None, None],
        [None, None],
        [None, None],
        [None, None],
        [None, None],
        [None, None],
        [None, None],
        [None, None],
        [None, None]
    ];

    let day = env::args().nth(1).expect("Must specify a day.")
        .parse::<usize>().expect("Day must be integer.");
    let part = env::args().nth(2).expect("Must specify a part.")
        .parse::<usize>().expect("Part must be an integer.");
    let input_fpath_str = env::args().nth(3)
        .expect("Must specify a path to input data.");
    let input_fpath = path::Path::new(&input_fpath_str);
    let func = &funcs[day-1][part-1];

    match func {
        Some(f) => {
            let start_time = Instant::now();
            let input = fs::read_to_string(input_fpath)
                .expect("Could not read input file.");
            let output = f(&input);
            let end_time = Instant::now();
            let duration = end_time - start_time;
            println!("Day {}, part {} answer:", day, part);
            println!("{}", output);
            println!("Time taken: {:#?}", duration)
        },
        None => panic!("No function found for day {}, part {}.", day, part)
    }
}

