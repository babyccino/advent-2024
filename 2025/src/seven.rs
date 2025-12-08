use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::util::{get_next, next_iter, previous_iter};

fn part_one() {
    let file = File::open("./input/seven/big.txt").unwrap();
    let reader = BufReader::new(file);

    let mut iter = reader.lines().step_by(2);
    let line = iter.next().unwrap().unwrap();
    let len = line.len();

    let pos = line.find('S').unwrap();
    let mut positions = vec![false; len];
    positions[pos] = true;

    let mut total = 0;
    for line in iter {
        let line = line.unwrap();
        for (i, c) in line.char_indices() {
            if c == '^' && positions[i] {
                total += 1;
                positions[i] = false;
                if i > 0 {
                    positions[i - 1] = true
                }
                if i < len - 1 {
                    positions[i + 1] = true
                }
            }
        }
    }

    println!("{}", total);
}

fn rec_old(
    len: usize,
    vecs: &[Vec<usize>],
    beam: usize,
    depth: usize,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if vecs.is_empty() {
        return 1;
    }

    let vec = &vecs[0];
    let new_vecs = &vecs[1..];

    if vec.iter().find(|el| **el == beam).is_some() {
        if let Some(total) = memo.get(&(beam, depth)) {
            // println!("{beam} {depth} memoised {total}");
            return *total;
        }

        let mut total = 0;
        if beam > 0 {
            total += rec_old(len, new_vecs, beam - 1, depth + 1, memo);
        }
        if beam < len - 1 {
            total += rec_old(len, new_vecs, beam + 1, depth + 1, memo);
        }

        memo.insert((beam, depth), total);
        // println!("total {total}");
        total
    } else {
        rec_old(len, new_vecs, beam, depth + 1, memo)
    }
}

fn part_two_old() -> u64 {
    let file = File::open("./input/seven/big.txt").unwrap();
    let reader = BufReader::new(file);

    let mut iter = reader.lines().step_by(2);
    let line = iter.next().unwrap().unwrap();
    let len = line.len();

    let pos = line.find('S').unwrap();

    let splits = iter
        .map(|line| {
            line.unwrap()
                .char_indices()
                .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // dbg!(&splits, pos);

    let mut memo = HashMap::new();
    rec_old(len, &splits[..], pos, 0, &mut memo)
}

fn rec(paths: Vec<u64>, mut lines: impl Iterator<Item = String>) -> u64 {
    let len = paths.len();
    let line = lines.next();
    let Some(line) = line else {
        return paths.into_iter().fold(0, |total, curr| total + curr);
    };

    let mut new_paths = vec![0; len];

    // dbg!(&new_paths);

    for (i, (curr_path_count, c)) in paths
        .into_iter()
        .zip(line.chars())
        .enumerate()
        .filter(|(_, (beam, _))| *beam > 0)
    {
        // dbg!(i, curr_path_count, c);
        if c == '^' {
            if i > 0 {
                new_paths[i - 1] += curr_path_count;
            }
            if i < len - 1 {
                new_paths[i + 1] = curr_path_count
            }
        } else {
            new_paths[i] += curr_path_count;
        }
    }
    // dbg!(line);
    // dbg!(&new_paths);

    rec(new_paths, lines)
}

fn rec_fp_masturbation(paths: Vec<u64>, lines: impl Iterator<Item = String>) -> u64 {
    let Some((line, new_lines)) = get_next(lines) else {
        return paths.into_iter().fold(0, |total, curr| total + curr);
    };

    let beam_and_char = || paths.iter().zip(line.chars());

    // dbg!(beam_and_char().collect::<Vec<_>>());

    let previous_next = previous_iter(beam_and_char()).zip(next_iter(beam_and_char()));

    fn map_tuple((beam, c): (&u64, char)) -> u64 {
        if c == '^' {
            *beam
        } else {
            0
        }
    }

    // dbg!(map_tuple((&1, '^')));

    let new_paths = beam_and_char()
        .zip(previous_next)
        .map(|((curr, c), (prev, next))| {
            let ret = if c == '^' { 0 } else { *curr }
                + prev.map_or(0, map_tuple)
                + next.map_or(0, map_tuple);
            // dbg!(curr, prev, next, ret);
            ret
        })
        .collect::<Vec<_>>();

    // dbg!(&new_paths);

    rec(new_paths, new_lines)
}

fn rec_fp_masturbation_no_iter(paths: Vec<u64>, lines: impl Iterator<Item = String>) -> u64 {
    let Some((line, new_lines)) = get_next(lines) else {
        return paths.into_iter().sum();
    };

    let new_paths = paths
        .iter()
        .zip(line.chars())
        .enumerate()
        .map(|(i, (curr, c))| {
            let mut ret = if c == '^' { 0 } else { *curr };
            if i > 0 {
                let i = i - 1;
                ret += line
                    .chars()
                    .nth(i)
                    .map_or(0, |c| if c == '^' { paths[i] } else { 0 });
            }
            if i < paths.len() - 1 {
                let i = i + 1;
                ret += line
                    .chars()
                    .nth(i)
                    .map_or(0, |c| if c == '^' { paths[i] } else { 0 });
            }
            ret
        })
        .collect::<Vec<_>>();

    // dbg!(&new_paths);

    rec(new_paths, new_lines)
}
fn part_two() -> u64 {
    let file = File::open("./input/seven/big.txt").unwrap();
    let reader = BufReader::new(file);

    let mut iter = reader.lines().step_by(2).map(|line| line.unwrap());
    let line = iter.next().unwrap();
    let len = line.len();

    let pos = line.find('S').unwrap();
    let mut beams = vec![0; len];
    beams[pos] = 1;

    rec(beams, iter)
}

fn part_two_fp() -> u64 {
    let file = File::open("./input/seven/big.txt").unwrap();
    let reader = BufReader::new(file);

    let mut iter = reader.lines().step_by(2).map(|line| line.unwrap());
    let line = iter.next().unwrap();
    let len = line.len();

    let pos = line.find('S').unwrap();
    let mut beams = vec![0; len];
    beams[pos] = 1;

    rec_fp_masturbation(beams, iter)
}

fn part_two_fp_no_iter() -> u64 {
    let file = File::open("./input/seven/big.txt").unwrap();
    let reader = BufReader::new(file);

    let mut iter = reader.lines().step_by(2).map(|line| line.unwrap());
    let line = iter.next().unwrap();
    let len = line.len();

    let pos = line.find('S').unwrap();
    let mut beams = vec![0; len];
    beams[pos] = 1;

    rec_fp_masturbation_no_iter(beams, iter)
}

pub fn day_seven() {
    use std::time::Instant;

    for _ in 0..100 {
        let old_total = part_two_old();
        let total = part_two();

        println!("{total} {old_total}");
    }

    let now = Instant::now();
    let total_old = part_two_old();
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
    println!("{total_old}");

    let now = Instant::now();
    let total = part_two();
    let elapsed = now.elapsed();

    println!("Elapsed not fp: {:.2?}", elapsed);
    println!("{total}");

    let now = Instant::now();
    let total = part_two_fp();
    let elapsed = now.elapsed();

    println!("Elapsed fp: {:.2?}", elapsed);
    println!("{total}");

    let now = Instant::now();
    let total = part_two_fp_no_iter();
    let elapsed = now.elapsed();

    println!("Elapsed fp no iter: {:.2?}", elapsed);
    println!("{total}");

    let now = Instant::now();
    let total = part_two();
    let elapsed = now.elapsed();

    println!("Elapsed not fp: {:.2?}", elapsed);
    println!("{total}");

    let now = Instant::now();
    let total = part_two_fp();
    let elapsed = now.elapsed();

    println!("Elapsed fp: {:.2?}", elapsed);
    println!("{total}");

    let now = Instant::now();
    let total = part_two_fp_no_iter();
    let elapsed = now.elapsed();

    println!("Elapsed fp no iter: {:.2?}", elapsed);
    println!("{total}");
}
