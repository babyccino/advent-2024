use std::cmp::Ordering;
use std::fs::File;
use std::io::{prelude::BufRead, BufReader};
use std::ops::Range;

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

pub fn get_data(file_path: &str) -> (Vec<Range<u64>>, Vec<u64>) {
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

    ranges.sort_unstable_by(order_ranges);

    data.sort();

    (ranges, data)
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

pub fn part_one(ranges: &[Range<u64>], data: &[u64]) -> u64 {
    in_overlaps(ranges, data)
}

pub fn part_two(ranges: &[Range<u64>]) -> u64 {
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

pub fn day_five() -> (u64, u64) {
    let (ranges, data) = get_data("./input/five/big.txt");
    let part_one = part_one(&ranges, &data);
    let part_two = part_two(&ranges);
    (part_one, part_two)
}
