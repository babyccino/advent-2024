use std::fs::File;
use std::io::{prelude::BufRead, BufReader};

fn get(line: &str) -> (u8, usize) {
    // dbg!(line);
    let mut max = 1;
    let mut pos = 0;
    for (i, c) in line.char_indices() {
        let val = c as u8 - '0' as u8;
        if val == 9 {
            return (val, i);
        }
        if val > max {
            (max, pos) = (val, i);
        }
    }
    (max, pos)
}

pub fn part_one() -> u32 {
    let file = File::open("./input/three/big.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines().fold(0 as u32, |result, line| {
        let line = line.unwrap();
        let (first, first_pos) = get(&line);
        if first == 1 {
            return 11;
        }
        if first_pos == line.len() - 1 {
            let second = first;
            let (first, _) = get(&line[..first_pos]);
            return result + (first * 10 + second) as u32;
        }

        let (second, _) = get(&line[(first_pos + 1)..]);
        return result + (first * 10 + second) as u32;
    })
}

fn perms(left: usize, line: &str, total: u64) -> u64 {
    if left == 0 {
        return total;
    }

    let first_slice = &line[..(line.len() - left + 1)];
    let (curr, pos) = get(first_slice);

    let end_slice = &line[(pos + 1)..];
    let curr_exp = (curr as u64) * (10 as u64).pow(left as u32 - 1);
    // println!("curr: {curr}, pos: {pos}, lef: {left}, line: {line}, slice: {slice}");

    perms(left - 1, end_slice, total + curr_exp)
}

pub fn part_two() -> u64 {
    let file = File::open("./input/three/big.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |result, line| result + perms(12, &line, 0))
}
