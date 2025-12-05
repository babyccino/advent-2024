use std::cmp::{max, Ordering};
use std::fs::File;
use std::io::{prelude::BufRead, BufReader};
use std::ops::Range;

fn line_to_range(line: String) -> Range<usize> {
    let dash_index = line.find('-').unwrap();
    let first = line[..dash_index].parse::<usize>().unwrap();
    let second = line[(dash_index + 1)..].parse::<usize>().unwrap();
    first..second
}

fn order_ranges(a: &Range<usize>, b: &Range<usize>) -> Ordering {
    let ord = a.start.cmp(&b.start);
    if ord == Ordering::Equal {
        a.end.cmp(&b.end)
    } else {
        ord
    }
}

fn get_ranges_only(file_path: &str) -> Vec<Range<usize>> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut ranges = reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.trim().is_empty())
        .map(line_to_range)
        .collect::<Vec<_>>();

    ranges.sort_unstable_by(order_ranges);

    ranges
}

fn get_data(file_path: &str) -> (Vec<Range<usize>>, Vec<usize>) {
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
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    ranges.sort_unstable_by(order_ranges);

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

        let end = max(curr.end, next.end);
        vec[i] = curr.start..end;

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
    let ranges = get_ranges_only("./input/five/big.txt");
    let ranges = remove_overlaps(ranges);
    return ranges
        .into_iter()
        .fold(0, |total, range| total + range.end - range.start + 1) as u64;
}
