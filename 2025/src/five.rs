use std::cmp::{max, Ordering};
use std::fs::File;
use std::io::{prelude::BufRead, BufReader};
use std::ops::Range;

use rand::prelude::*;
use rand::rngs::ThreadRng;

fn line_to_range(line: String) -> Range<u64> {
    let dash_index = line.find('-').unwrap();
    let first = line[..dash_index].parse::<u64>().unwrap();
    let second = line[(dash_index + 1)..].parse::<u64>().unwrap();
    first..second
}

fn order_ranges(a: &Range<u64>, b: &Range<u64>) -> Ordering {
    let ord = a.start.cmp(&b.start);
    if ord == Ordering::Equal {
        a.end.cmp(&b.end)
    } else {
        ord
    }
}

fn get_data(file_path: &str) -> (Vec<Range<u64>>, Vec<u64>) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut ranges = lines
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .map(line_to_range)
        .collect::<Vec<_>>();
    let mut data = lines
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    (ranges, data)
}

fn remove_overlaps(mut vec: Vec<Range<u64>>) -> Vec<Range<u64>> {
    if vec.len() == 1 {
        return vec;
    }

    let mut i = 0;
    while i + 1 < vec.len() {
        let curr = &vec[i];
        let next = &vec[i + 1];
        if next.start > curr.end {
            i += 1;
            continue;
        }

        let end = max(curr.end, next.end);
        vec[i] = curr.start..end;

        vec.remove(i + 1);
    }

    vec
}

fn in_overlaps(ranges: &[Range<u64>], data: &[u64]) -> u64 {
    in_overlaps_inner(ranges, data, 0)
}
fn in_overlaps_inner(ranges: &[Range<u64>], data: &[u64], total: u64) -> u64 {
    if data.len() == 0 || ranges.len() == 0 {
        return total;
    }

    let range = &ranges[0];
    let curr = data[0];

    if curr < range.start {
        return in_overlaps_inner(ranges, &data[1..], total);
    } else if range.end < curr {
        return in_overlaps_inner(&ranges[1..], data, total);
    }

    return in_overlaps_inner(ranges, &data[1..], total + 1);
}

fn part_one_old(ranges: &[Range<u64>], data: &[u64]) -> u64 {
    in_overlaps(ranges, data)
}

fn part_two_old(ranges: &[Range<u64>]) -> u64 {
    return ranges
        .into_iter()
        .fold(0, |total, range| total + range.end - range.start + 1) as u64;
}

fn part_one(ranges: &[Range<u64>], data: &[u64]) -> u64 {
    in_overlaps(ranges, data)
}

fn part_two(ranges: &[Range<u64>]) -> u64 {
    let first = ranges[0].end - ranges[0].start + 1;
    let (rest, _) = ranges[1..]
        .iter()
        .fold((0, ranges[0].clone()), |(total, prev), curr| {
            let curr_len = curr.end - curr.start + 1;

            if curr.start > prev.end {
                return (total + curr_len, curr.clone());
            } else if curr.end <= prev.end {
                return (total, prev);
            }

            let intersetion = prev.end - curr.start + 1;
            (total + curr_len - intersetion, curr.clone())
        });
    first + rest
}

fn get_data_rand(
    ranges: &[Range<u64>],
    data: &[u64],
    rng: &mut ThreadRng,
) -> (Vec<Range<u64>>, Vec<u64>) {
    let mut ranges = Vec::from(ranges);
    let mut data = Vec::from(data);
    ranges[0] = rng.random_range(0..100)..rng.random_range(0..100);
    data[0] = rng.random_range(0..100);
    (ranges, data)
}

const TOTAL_RUNS: usize = 2000000;

/*
real 18.00
user 17.73
sys 0.02
             1556480  maximum resident set size
                 247  page reclaims
                   1  page faults
                   2  voluntary context switches
                 103  involuntary context switches
        335522887495  instructions retired
         66551528478  cycles elapsed
             1065224  peak memory footprint
 */
pub fn day_five_old() {
    let (ranges, data) = get_data("./input/five/big.txt");
    let mut counter = 0;
    let mut rng = rand::rng();

    let (part_one, part_two, total) = (0..TOTAL_RUNS).fold((0, 0, 0), |(_, _, total), i| {
        let (mut ranges, mut data) = get_data_rand(&ranges, &data, &mut rng);
        ranges.sort_unstable_by(order_ranges);
        data.sort();

        let ranges = remove_overlaps(ranges);
        let part_one = part_one_old(&ranges, &data);
        let part_two = part_two_old(&ranges);
        if i % (TOTAL_RUNS / 10) == 0 {
            println!("{counter}0% done");
            counter += 1;
        }
        (part_one, part_two, part_one + part_two + total)
    });

    println!("day one, p1: {} should be 862", part_one);
    println!("day one, p1: {} should be 357907198933892", part_two);
    println!("{total}");
}

/*
real 15.64
user 15.21
sys 0.03
             1556480  maximum resident set size
                 247  page reclaims
                   1  page faults
                   6  voluntary context switches
                  63  involuntary context switches
        281562545720  instructions retired
         57165319628  cycles elapsed
             1065224  peak memory footprint
 */
pub fn day_five() {
    let (ranges, data) = get_data("./input/five/big.txt");
    let mut counter = 0;
    let mut rng = rand::rng();

    let (part_one, part_two, total) = (0..TOTAL_RUNS).fold((0, 0, 0), |(_, _, total), i| {
        let (mut ranges, mut data) = get_data_rand(&ranges, &data, &mut rng);
        ranges.sort_unstable_by(order_ranges);
        data.sort();

        let part_one = part_one(&ranges, &data);
        let part_two = part_two(&ranges);
        if i % (TOTAL_RUNS / 10) == 0 {
            println!("{counter}0% done");
            counter += 1;
        }
        (part_one, part_two, part_one + part_two + total)
    });

    println!("day one, p1: {} should be 862", part_one);
    println!("day one, p1: {} should be 357907198933892", part_two);
    println!("{total}");
}
