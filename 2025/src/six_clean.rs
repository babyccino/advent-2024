use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul},
};

#[repr(u8)]
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

    let init = (Vec::new(), Vec::new());
    let (nums, ops) =
        reader
            .lines()
            .map(|line| line.unwrap())
            .fold(init, |(mut nums, ops), line| {
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

fn get_operations(slice: &str) -> Vec<(Operation, usize)> {
    get_operations_inner(slice, Vec::new())
}
fn get_operations_inner(slice: &str, mut ops: Vec<(Operation, usize)>) -> Vec<(Operation, usize)> {
    let check = &['*', '+'][..];
    let c = slice.chars().next().unwrap();

    if let Some(index) = slice[1..].find(check) {
        let index = index + 1;
        ops.push((Operation::parse(c), index - 1));
        get_operations_inner(&slice[index..], ops)
    } else {
        ops.push((Operation::parse(c), slice.len()));
        ops
    }
}

fn get_data_two(file_path: &str) -> (Vec<u8>, Vec<(Operation, usize)>, usize) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let (nums, ops, line_len) = reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.trim().is_empty())
        .fold((Vec::new(), Vec::new(), 0), |(mut nums, ops, _), line| {
            if line.find('*').is_none() && line.find('+').is_none() {
                nums.extend(
                    line.chars()
                        .map(|el| if el == ' ' { 0 } else { el as u8 - '0' as u8 }),
                );
                return (nums, ops, line.len());
            }

            (nums, get_operations(&line[..]), line.len())
        });

    (nums, ops, line_len)
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

pub fn run() {
    let (nums_one, ops_one) = get_data("./input/six/big.txt");
    let (nums_two, ops_two, line_len) = get_data_two("./input/six/big.txt");

    {
        let part_one = part_one(&nums_one, &ops_one);
        assert_eq!(part_one, ans);
        let part_two = part_two(&nums_two, &ops_two, line_len);
        assert_eq!(part_two, ans);
    }
}
