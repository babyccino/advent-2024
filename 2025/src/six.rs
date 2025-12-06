use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul},
};

use rand::prelude::*;
use rand::rngs::ThreadRng;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum Operation {
    Mult = '*' as u8,
    Add = '+' as u8,
}

impl Operation {
    fn operate<T: Add<Output = T> + Mul<Output = T> + Display>(&self, a: T, b: T) -> T {
        match self {
            Self::Mult => a * b,
            Self::Add => a + b,
        }
    }

    fn parse(el: char) -> Self {
        match el {
            '*' => Self::Mult,
            '+' => Self::Add,
            _ => panic!("oops"),
        }
    }
}

fn get_data(file_path: &str) -> (Vec<u16>, Vec<Operation>) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|line| line.unwrap());
    let (nums, ops) = lines.fold((Vec::new(), Vec::new()), |(mut nums, ops), line| {
        let line = line.trim();
        if line.chars().next().unwrap().is_ascii_digit() {
            nums.extend(line.split_whitespace().map(|el| el.parse::<u16>().unwrap()));
            return (nums, ops);
        }

        let ops = line
            .split_whitespace()
            .map(|el| Operation::parse(el.chars().next().unwrap()))
            .collect::<Vec<_>>();

        (nums, ops)
    });

    (nums, ops)
}

fn part_one(nums: &[u16], ops: &[Operation]) -> u64 {
    let iter = ops
        .iter()
        .enumerate()
        .map(|(i, _)| nums[i..].iter().step_by(ops.len()));
    ops.iter().zip(iter).fold(0, |total, (op, mut nums)| {
        let first = *nums.next().unwrap() as u64;
        let res = nums.fold(first, |total, a| op.operate(total, *a as u64));
        total + res
    })
}

fn part_one_old(nums: &[u16], ops: &[Operation]) -> u64 {
    part_one(nums, ops)
}

fn get_data_two(file_path: &str) -> (Vec<u8>, Vec<(Operation, usize)>, usize) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|line| line.unwrap());
    let (nums, ops, line_len) = lines.take_while(|line| !line.trim().is_empty()).fold(
        (Vec::new(), Vec::new(), 0),
        |(mut nums, ops, _), line| {
            if line.find('*').is_none() && line.find('+').is_none() {
                nums.extend(
                    line.chars()
                        .map(|el| if el == ' ' { 0 } else { el as u8 - '0' as u8 }),
                );
                return (nums, ops, line.len());
            }

            let check = &['*', '+'][..];
            let mut slice = &line[..];
            let mut ops = Vec::new();
            loop {
                let c = slice.chars().next().unwrap();
                if let Some(index) = slice[1..].find(check) {
                    let index = index + 1;
                    ops.push((Operation::parse(c), index - 1));
                    slice = &slice[index..];
                } else {
                    ops.push((Operation::parse(c), slice.len()));
                    break;
                }
            }

            (nums, ops, line.len())
        },
    );

    (nums, ops, line_len)
}

fn part_two_old(nums: &[u8], ops: &[(Operation, usize)], line_len: usize) -> u64 {
    let lens = ops.iter().map(|(_, len)| len);
    let ops = ops.iter().map(|(op, _)| op);
    let (_, iter) = lens.fold((0, Vec::new()), |(offset, mut vec), len| {
        let to_operate_on = (0..*len).map(move |len_offset| {
            let offset = offset + len_offset;
            nums[offset..]
                .iter()
                .step_by(line_len)
                .filter(|el| **el != 0)
                .fold(0, |total, curr| total * 10 + (*curr as u64))
        });
        vec.push(to_operate_on);
        (offset + len + 1, vec)
    });

    ops.zip(iter).fold(0, |total, (op, mut nums)| {
        let first = nums.next().unwrap();
        let res = nums.fold(first, |total, a| op.operate(total, a));
        total + res
    })
}

fn num_at(nums: &[u8], offset: usize, line_len: usize) -> u16 {
    nums[offset..]
        .iter()
        .step_by(line_len)
        .filter(|el| **el != 0)
        .fold(0, |total, curr| total * 10 + (*curr as u16))
}

fn nums_at<'a>(
    nums: &'a [u8],
    len: usize,
    offset: usize,
    line_len: usize,
) -> impl Iterator<Item = u16> + 'a {
    (0..len).map(move |len_offset| num_at(nums, offset + len_offset, line_len))
}

fn part_two(nums: &[u8], ops: &[(Operation, usize)], line_len: usize) -> u64 {
    let init = (0 as u64, 0 as usize);
    let (total, _) = ops.iter().fold(init, |(total, offset), (op, len)| {
        let mut to_operate_on = nums_at(nums, *len, offset, line_len);
        let first = to_operate_on.next().unwrap() as u64;
        let res = to_operate_on.fold(first, |total, a| op.operate(total, a as u64));
        (total + res, offset + len + 1)
    });
    total
}

const TOTAL_RUNS: usize = 1000000;

/*
real 12.37
user 11.80
sys 0.17
             1884160  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                 267  page reclaims
                   1  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   2  voluntary context switches
                 325  involuntary context switches
        229297140797  instructions retired
         44692262081  cycles elapsed
             1409312  peak memory footprint
 */
pub fn day_six_old() {
    let (mut nums_one, ops_one) = get_data("./input/six/big.txt");
    let (mut nums_two, ops_two, line_len) = get_data_two("./input/six/big.txt");

    {
        let part_one = part_one_old(&nums_one, &ops_one);
        assert_eq!(part_one, 6635273135233);
        let part_two = part_two_old(&nums_two, &ops_two, line_len);
        assert_eq!(part_two, 12542543681221);
    }

    let mut counter = 0;
    let mut rng = rand::rng();

    let (part_one, part_two, total) = (0..TOTAL_RUNS).fold((0, 0, 0), |(_, _, total), i| {
        rand_data(&mut nums_one, &mut nums_two, &mut rng);

        let part_one = part_one_old(&nums_one, &ops_one);
        let part_two = part_two_old(&nums_two, &ops_two, line_len);
        if i % (TOTAL_RUNS / 10) == 0 {
            println!("{counter}0% done");
            counter += 1;
        }
        (part_one, part_two, part_one + part_two + total)
    });

    println!("{total}");
}

fn rand_data(nums_one: &mut [u16], nums_two: &mut [u8], rng: &mut ThreadRng) {
    nums_one[0] = rng.random_range(0..100);
    nums_two[0] = rng.random_range(0..100);
}

/*
real 9.15
user 8.75
sys 0.03
             1540096  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                 246  page reclaims
                   1  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   2  voluntary context switches
                 498  involuntary context switches
        174428487248  instructions retired
         32720961742  cycles elapsed
             1048840  peak memory footprint
 */
pub fn day_six() {
    let (mut nums_one, ops_one) = get_data("./input/six/big.txt");
    let (mut nums_two, ops_two, line_len) = get_data_two("./input/six/big.txt");

    {
        let part_one = part_one(&nums_one, &ops_one);
        assert_eq!(part_one, 6635273135233);
        let part_two = part_two(&nums_two, &ops_two, line_len);
        assert_eq!(part_two, 12542543681221);
    }

    let mut counter = 0;
    let mut rng = rand::rng();

    let (part_one, part_two, total) = (0..TOTAL_RUNS).fold((0, 0, 0), |(_, _, total), i| {
        rand_data(&mut nums_one, &mut nums_two, &mut rng);

        let part_one = part_one(&nums_one, &ops_one);
        let part_two = part_two(&nums_two, &ops_two, line_len);
        if i % (TOTAL_RUNS / 10) == 0 {
            println!("{counter}0% done");
            counter += 1;
        }
        (part_one, part_two, part_one + part_two + total)
    });

    println!("{total}");
}
