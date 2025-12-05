use std::fs::File;
use std::io::{prelude::BufRead, BufReader};

use crate::util::real_mod;

fn dir(line: &str) -> i32 {
    let char = line.trim().chars().next().unwrap();
    if char == 'L' {
        -1
    } else {
        1
    }
}

pub fn part_one() -> i32 {
    let file = File::open("./input/one/small.txt").unwrap();
    let reader = BufReader::new(file);
    let dial_size = 100;
    let midpoint = dial_size / 2;

    let (result, _) = reader.lines().fold((0, midpoint), |(result, dial), line| {
        let line = line.unwrap();
        if line.is_empty() {
            return (result, dial);
        }

        let dir = dir(&line);

        let incr = line[1..].parse::<i32>().unwrap();

        let dial = real_mod(dial + (dir * incr), dial_size);
        let result = result + if dial == 0 { 1 } else { 0 };
        (result, dial)
    });

    result
}

pub fn part_two() -> i32 {
    let file = File::open("./input/one/big.txt").unwrap();
    let reader = BufReader::new(file);
    let dial_size = 100;

    let (result, _) = reader.lines().fold((0, 50), |(result, dial), line| {
        let line = line.unwrap();
        if line.is_empty() {
            return (result, dial);
        }

        let dir = dir(&line);

        let incr = line[1..].parse::<i32>().unwrap();
        let new_dial = real_mod(dial + (dir * incr), dial_size);

        let full_turns = incr / dial_size;
        let after_leftover = dial + (dir * (incr - full_turns * dial_size));
        let went_past_zero =
            (dial != 0 && (after_leftover <= 0 || after_leftover >= dial_size)) as i32;

        let new_result = result + full_turns + went_past_zero;

        (new_result, new_dial)
    });

    result
}
