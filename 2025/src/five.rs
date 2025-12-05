use std::cmp::{max, Ordering};
use std::fs::File;
use std::io::{prelude::BufRead, BufReader};
use std::ops::Range;

fn get_data(file_path: &str) -> (Vec<Range<usize>>, Vec<usize>) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let init = (Vec::new(), Vec::new());
    let (mut ranges, mut data) =
        reader
            .lines()
            .map(|line| line.unwrap())
            .fold(init, |(mut ranges, mut data), line| {
                let line = line.trim();

                if line.is_empty() {
                    return (ranges, data);
                }

                let dash_index = line.find('-');
                if let Some(dash_index) = dash_index {
                    let first = line[..dash_index].parse::<usize>().unwrap();
                    let second = line[(dash_index + 1)..].parse::<usize>().unwrap();
                    ranges.push(first..second);
                } else {
                    data.push(line.parse::<usize>().unwrap())
                }

                (ranges, data)
            });

    ranges.sort_unstable_by(|a, b| {
        let ord = a.start.cmp(&b.start);
        if ord == Ordering::Equal {
            a.end.cmp(&b.end)
        } else {
            ord
        }
    });

    data.sort();

    (ranges, data)
}

fn remove_overlaps(mut vec: Vec<Range<usize>>) -> Vec<Range<usize>> {
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

        vec[i] = curr.start..max(curr.end, next.end);

        vec.remove(i + 1);
    }

    vec
}

fn in_overlaps(ranges: &[Range<usize>], data: &[usize]) -> usize {
    in_overlaps_inner(ranges, data, 0)
}
fn in_overlaps_inner(ranges: &[Range<usize>], data: &[usize], total: usize) -> usize {
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

pub fn part_one() -> u64 {
    let (ranges, data) = get_data("./input/five/big.txt");
    let ranges = remove_overlaps(ranges);
    in_overlaps(&ranges, &data) as u64
}

pub fn part_two() -> u64 {
    let (ranges, _) = get_data("./input/five/big.txt");
    let ranges = remove_overlaps(ranges);
    return ranges
        .into_iter()
        .fold(0, |total, range| total + range.end - range.start + 1) as u64;
}
