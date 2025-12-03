use std::fs::File;
use std::io::{prelude::BufRead, BufReader};

pub fn part_one() -> i32 {
    let file = File::open("./input/one/small.txt").unwrap();
    let reader = BufReader::new(file);

    let (result, _) = reader.lines().fold((0, 50), |(result, dial), line| {
        let line = line.unwrap();
        if line.is_empty() {
            return (result, dial);
        }

        let dir = if line.trim().chars().next().unwrap() == 'L' {
            -1
        } else {
            1
        };

        let incr = line[1..].parse::<i32>().unwrap();

        let dial = ((dial + (dir * incr)) % 100 + 100) % 100;
        let result = result + if dial == 0 { 1 } else { 0 };
        (result, dial)
    });

    result
}

pub fn part_two() -> i32 {
    let file = File::open("./input/one/big.txt").unwrap();
    let reader = BufReader::new(file);

    let (result, _) = reader.lines().fold((0, 50), |(result, dial), line| {
        let line = line.unwrap();
        if line.is_empty() {
            return (result, dial);
        }

        let dir = if line.trim().chars().next().unwrap() == 'L' {
            -1
        } else {
            1
        };

        let incr = line[1..].parse::<i32>().unwrap();
        let new_dial = ((dial + (dir * incr)) % 100 + 100) % 100;

        let full_turns = incr / 100;
        let after_leftover = dial + (dir * (incr - full_turns * 100));
        let went_past_zero = (dial != 0 && (after_leftover <= 0 || after_leftover >= 100)) as i32;

        let new_result = result + full_turns + went_past_zero;

        (new_result, new_dial)
    });

    result
}
