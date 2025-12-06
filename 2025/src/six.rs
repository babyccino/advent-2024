use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul},
};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum Operation {
    Mult = '*' as u8,
    Add = '+' as u8,
}

impl Operation {
    fn operate<T: Add<Output = T> + Mul<Output = T> + Display>(&self, a: T, b: T) -> T {
        // println!("{} on {}, {}", *self as u8, a, b);
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

fn get_data(file_path: &str) -> (Vec<u64>, Vec<Operation>) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|line| line.unwrap());
    let (nums, ops) = lines.fold((Vec::new(), Vec::new()), |(mut nums, ops), line| {
        let line = line.trim();
        if line.chars().next().unwrap().is_ascii_digit() {
            nums.extend(line.split_whitespace().map(|el| el.parse::<u64>().unwrap()));
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

fn part_one(nums: &[u64], ops: &[Operation]) -> u64 {
    let iter = ops
        .iter()
        .enumerate()
        .map(|(i, _)| nums[i..].iter().step_by(ops.len()));
    ops.iter().zip(iter).fold(0, |total, (op, mut nums)| {
        let first = *nums.next().unwrap();
        let res = nums.fold(first, |total, a| op.operate(total, *a));
        total + res
    })
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

fn part_two(nums: &[u8], ops: &[(Operation, usize)], line_len: usize) -> u64 {
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

pub fn day_six() {
    // let (nums, ops) = get_data("./input/six/big.txt");
    // let res = part_one(&nums, &ops);
    let (nums, ops, line_len) = get_data_two("./input/six/big.txt");
    let res = part_two(&nums, &ops, line_len);
    let expected: u64 = 7226 * 2568 * 4785 + (2715 + 3251 + 818 + 41);
    println!("{}, supposed to be {}", res, expected);
}
