use std::fs::read_to_string;

use rand::prelude::*;
use rand::rngs::ThreadRng;

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

fn check_num_two_old(num: u64) -> u64 {
    // println!("\nchecking: {num}");
    let str = num.to_string();
    let mut div = 1;

    while div <= str.len() / 2 {
        // println!("div: {div}");
        if str.len() % div != 0 {
            div = div + 1;
            continue;
        }

        let slice = &str[..div];
        // println!("slice: {slice}");

        let mut correct = true;
        for i in 1..(str.len() / div) {
            let check = &str[i * div..(i + 1) * div];

            if slice != check {
                correct = false;
                break;
            }
        }

        if correct {
            // println!("{num}, {div}, yes");
            return num;
        }

        div = div + 1;
    }

    0
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

fn check_num_old(num: u64) -> u64 {
    let str = num.to_string();
    if str.len() % 2 == 0 {
        let first = &str[..str.len() / 2];
        let second = &str[str.len() / 2..];
        if first == second {
            num
        } else {
            0
        }
    } else {
        0
    }
}

fn check_range_old(slice: &str) -> u64 {
    let dash = slice.find('-').unwrap();
    let start_slice = &slice[..dash];
    let Ok(start) = start_slice.parse::<u64>() else {
        panic!("failed parsing {start_slice}");
    };
    let end_slice = &slice[(dash + 1)..];
    let Ok(end) = end_slice.parse::<u64>() else {
        panic!("failed parsing {end_slice}");
    };
    (start..=end).fold(0, |total, num| total + check_num_old(num))
}

fn check_range_two_old(slice: &str) -> u64 {
    let dash = slice.find('-').unwrap();

    let end = slice[(dash + 1)..].parse::<u64>().unwrap();
    if end < 11 {
        return 0;
    }

    let start = slice[..dash].parse::<u64>().unwrap();
    let start = if start < 11 { 11 } else { start };

    (start..end).fold(0, |total, num| total + check_num_two_old(num))
}

pub fn part_one(file: &str) -> u64 {
    file.split(',')
        .fold(0, |total, range| total + check_range(range.trim()))
}
pub fn part_two(file: &str) -> u64 {
    file.split(',')
        .fold(0, |total, range| total + check_range_two(range.trim()))
}

pub fn part_one_old(file: &str) -> u64 {
    file.split(',')
        .fold(0, |total, range| total + check_range_old(range.trim()))
}
pub fn part_two_old(file: &str) -> u64 {
    file.split(',')
        .fold(0, |total, range| total + check_range_two_old(range.trim()))
}

fn get_data_rand(str: &str, rng: &mut ThreadRng) -> String {
    let mut str = String::from(str);
    unsafe { str.as_bytes_mut()[0] = rng.random_range('1'..'9') as u8 }
    str
}

const TOTAL_RUNS: usize = 100;

/*
real 14.50
user 14.18
sys 0.01
             1490944  maximum resident set size
                 243  page reclaims
                   1  page faults
                   1  voluntary context switches
                  84  involuntary context switches
        326763285649  instructions retired
         53205997062  cycles elapsed
             1016072  peak memory footprint
 */
pub fn day_two_old() {
    let file = read_to_string("./input/two/big.txt").unwrap();
    let mut counter = 0;
    let mut rng = rand::rng();

    let (part_one, part_two, total) = (0..TOTAL_RUNS).fold((0, 0, 0), |(_, _, total), i| {
        let file = get_data_rand(&file, &mut rng);

        let part_one = part_one_old(&file);
        let part_two = part_two_old(&file);
        if i % (TOTAL_RUNS / 10) == 0 {
            println!("{counter}0% done");
            counter += 1;
        }
        (part_one, part_two, part_one + part_two + total)
    });

    println!("day two, p1: {} ", part_one);
    println!("day two, p1: {} ", part_two);
    println!("{total}");
}

/*
real 4.93
user 4.62
sys 0.01
             1474560  maximum resident set size
                 242  page reclaims
                   1  page faults
                   1  voluntary context switches
                 242  involuntary context switches
         97388442208  instructions retired
         17718218560  cycles elapsed
              983304  peak memory footprint
 */
pub fn day_two() {
    let file = read_to_string("./input/two/big.txt").unwrap();
    let mut counter = 0;
    let mut rng = rand::rng();

    let (part_one, part_two, total) = (0..TOTAL_RUNS).fold((0, 0, 0), |(_, _, total), i| {
        let file = get_data_rand(&file, &mut rng);

        let part_one = part_one(&file);
        let part_two = part_two(&file);
        if i % (TOTAL_RUNS / 10) == 0 {
            println!("{counter}0% done");
            counter += 1;
        }
        (part_one, part_two, part_one + part_two + total)
    });

    println!("day two, p1: {} ", part_one);
    println!("day two, p1: {} ", part_two);
    println!("{total}");
}
