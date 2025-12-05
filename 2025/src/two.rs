use std::fs::read_to_string;

fn num_len(num: u64) -> u64 {
    num_len_inner(num, 1)
}
fn num_len_inner(num: u64, total: u64) -> u64 {
    let res = num / 10;
    if res == 0 {
        total
    } else {
        num_len_inner(res, total + 1)
    }
}

fn check_num(num: u64) -> u64 {
    let num_len = num_len(num);
    if num_len % 2 != 0 {
        return 0;
    }

    let exp = (10 as u64).pow(num_len as u32 / 2);
    let first = num % exp;
    let second = num / exp;
    if first == second {
        num
    } else {
        0
    }
}

fn check_range(slice: &str) -> u64 {
    let dash = slice.find('-').unwrap();
    let start_slice = &slice[..dash];
    let start = start_slice.parse::<u64>().unwrap();
    let end_slice = &slice[(dash + 1)..];
    let end = end_slice.parse::<u64>().unwrap();
    (start..=end).fold(0, |total, num| total + check_num(num))
}

pub fn part_one() -> u64 {
    let file = read_to_string("./input/two/big.txt").unwrap();
    file.split(',')
        .fold(0, |total, range| total + check_range(range.trim()))
}

fn check_num_two(num: u64) -> u64 {
    check_num_two_inner(num, 1)
}
fn check_num_two_inner(num: u64, div: u64) -> u64 {
    let num_len = num_len(num);
    if div > num_len / 2 {
        return 0;
    }

    if num_len % div != 0 {
        return check_num_two_inner(num, div + 1);
    }

    if check_repeating(num, div) {
        num
    } else {
        check_num_two_inner(num, div + 1)
    }
}

fn check_repeating(num: u64, div: u64) -> bool {
    let exp = (10 as u64).pow(div as u32);
    let check = num % exp;
    check_repeating_inner(check, exp, num)
}
fn check_repeating_inner(check: u64, exp: u64, num: u64) -> bool {
    if num == 0 {
        true
    } else if num % exp != check {
        false
    } else {
        check_repeating_inner(check, exp, num / exp)
    }
}

fn check_range_two(slice: &str) -> u64 {
    let dash = slice.find('-').unwrap();

    let end = slice[(dash + 1)..].parse::<u64>().unwrap();
    if end < 11 {
        return 0;
    }

    let start = slice[..dash].parse::<u64>().unwrap();
    let start = if start < 11 { 11 } else { start };

    (start..end).fold(0, |total, num| total + check_num_two(num))
}

pub fn part_two() -> u64 {
    let file = read_to_string("./input/two/big.txt").unwrap();
    file.split(',')
        .fold(0, |total, range| total + check_range_two(range.trim()))
}
