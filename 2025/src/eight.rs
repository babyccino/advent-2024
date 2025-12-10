use std::cmp::{Ordering, max, min};
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use itertools::Itertools;

use crate::util::Point3d;

fn part_one() -> usize {
    let file = File::open("./input/eight/big.txt").unwrap();
    let reader = BufReader::new(file);

    let points = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse::<usize>().unwrap();
            let y = iter.next().unwrap().parse::<usize>().unwrap();
            let z = iter.next().unwrap().parse::<usize>().unwrap();
            Point3d { x, y, z }
        })
        .collect::<Vec<_>>();

    let mut distances = points
        .iter()
        .enumerate()
        .flat_map(|(i, point)| {
            points
                .iter()
                .skip(i + 1)
                .map(move |other| (point, other, point.dist2(other)))
        })
        .collect_vec();

    distances.sort_by(|a, b| a.2.cmp(&b.2));

    let mut groups = Vec::<Vec<Point3d>>::new();

    let mut to_add = 1000;

    for (p1, p2, dist) in distances {
        // dbg!(dist);
        let found_p1 = groups
            .iter()
            .find_position(|group| group.iter().find(|el| (**el).eq(p1)).is_some())
            .map(|(i, _)| i);
        let found_p2 = groups
            .iter()
            .find_position(|group| group.iter().find(|el| (**el).eq(p2)).is_some())
            .map(|(i, _)| i);

        if let Some(found_p1) = found_p1
            && let Some(found_p2) = found_p2
        {
            if found_p1 == found_p2 {
                to_add -= 1;
            } else {
                let index = min(found_p1, found_p2);
                let other_index = max(found_p1, found_p2);

                let mut merger = groups.remove(other_index);
                groups[index].append(&mut merger);
                to_add -= 1;
            }
        } else if let Some(found_p1) = found_p1 {
            let group = &mut groups[found_p1];
            group.push(*p2);
            to_add -= 1;
        } else if let Some(found_p2) = found_p2 {
            let group = &mut groups[found_p2];
            group.push(*p1);
            to_add -= 1;
        } else {
            groups.push(vec![*p1, *p2]);
            to_add -= 1;
        }

        if to_add == 0 {
            break;
        }
    }

    groups.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
    // dbg!(&groups);

    groups
        .iter()
        .take(3)
        .fold(1, |total, curr| total * curr.len())
}

struct Ans<'a> {
    pub p1: &'a Point3d,
    pub p2: &'a Point3d,
    pub dist: usize,
}

impl Ord for Ans<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

impl PartialOrd for Ans<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Ans<'_> {}

impl PartialEq for Ans<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

fn part_two() -> usize {
    let file = File::open("./input/eight/big.txt").unwrap();
    let reader = BufReader::new(file);

    let points = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse::<usize>().unwrap();
            let y = iter.next().unwrap().parse::<usize>().unwrap();
            let z = iter.next().unwrap().parse::<usize>().unwrap();
            Point3d { x, y, z }
        })
        .collect::<Vec<_>>();

    let distances_iter = points.iter().enumerate().flat_map(|(i, point)| {
        points.iter().skip(i + 1).map(|other| Ans {
            p1: point,
            p2: other,
            dist: point.dist2(other),
        })
    });

    let mut distances2 = BinaryHeap::from_iter(distances_iter);

    let mut groups = Vec::<Vec<Point3d>>::new();

    let mut set: HashSet<Point3d, _> = points.iter().map(|el| *el).collect::<HashSet<Point3d>>();

    for Ans { p1, p2, dist } in std::iter::from_fn(move || distances2.pop()) {
        // dbg!(dist);
        let found_p1 = groups
            .iter()
            .find_position(|group| group.iter().find(|el| (**el).eq(p1)).is_some())
            .map(|(i, _)| i);
        let found_p2 = groups
            .iter()
            .find_position(|group| group.iter().find(|el| (**el).eq(p2)).is_some())
            .map(|(i, _)| i);

        set.remove(p1);
        set.remove(p2);

        if let Some(found_p1) = found_p1
            && let Some(found_p2) = found_p2
        {
            if found_p1 != found_p2 {
                let index = min(found_p1, found_p2);
                let other_index = max(found_p1, found_p2);

                let mut merger = groups.remove(other_index);
                groups[index].append(&mut merger);
            }
        } else if let Some(found_p1) = found_p1 {
            let group = &mut groups[found_p1];
            group.push(*p2);
        } else if let Some(found_p2) = found_p2 {
            let group = &mut groups[found_p2];
            group.push(*p1);
        } else {
            groups.push(vec![*p1, *p2]);
        }

        if set.is_empty() && groups.len() == 1 {
            return p1.x * p2.x;
        }
    }

    !unreachable!()
}

pub fn day_eight() {
    let now = Instant::now();
    let res = part_one();
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
    println!("{res}");

    let now = Instant::now();
    let res = part_two();
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
    println!("{res}");
}
