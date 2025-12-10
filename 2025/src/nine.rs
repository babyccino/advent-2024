use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use itertools::Itertools;
use plotters::prelude::BitMapBackend;
use plotters::prelude::*;

use crate::util::{BoundingBox, CardinalDirection, Point, cartesian_ranges, get_next};

fn part_one() -> usize {
    let file = File::open("./input/nine/big.txt").unwrap();
    let reader = BufReader::new(file);

    let points = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse::<usize>().unwrap();
            let y = iter.next().unwrap().parse::<usize>().unwrap();
            Point { x, y }
        })
        .collect::<Vec<_>>();

    points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            points[i + 1..]
                .iter()
                .map(move |p2| (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1))
        })
        .max()
        .unwrap()
}

fn check_points(curr: &Point, next: &Point, bounds: &BoundingBox) -> bool {
    if bounds.inside(*curr) {
        return false;
    }

    let vec = curr.to(*next);
    let dir = vec.is_cardinal().unwrap();

    if curr.eq(&bounds.tl()) {
        return dir != CardinalDirection::Down;
    }
    if curr.eq(&bounds.bl()) {
        return dir != CardinalDirection::Right;
    }
    if curr.eq(&bounds.tr()) {
        return dir != CardinalDirection::Left;
    }
    if curr.eq(&bounds.br()) {
        return dir != CardinalDirection::Up;
    }

    let in_y = bounds.inside_y(*curr);
    let in_x = bounds.inside_x(*curr);

    let res = if curr.x == bounds.x_min && in_y {
        dir != CardinalDirection::Right && dir != CardinalDirection::Down
    } else if curr.x == bounds.x_max && in_y {
        dir != CardinalDirection::Left && dir != CardinalDirection::Up
    } else if curr.y == bounds.y_min && in_x {
        dir != CardinalDirection::Down && dir != CardinalDirection::Left
    } else if curr.y == bounds.y_max && in_x {
        dir != CardinalDirection::Up && dir != CardinalDirection::Right
    } else if in_y
        && ((curr.x <= bounds.x_min && next.x >= bounds.x_max)
            || (next.x <= bounds.x_min && curr.x >= bounds.x_max))
    {
        false
    } else if in_x
        && ((curr.y <= bounds.y_min && next.y >= bounds.y_max)
            || (next.y <= bounds.y_min && curr.y >= bounds.y_max))
    {
        false
    } else {
        true
    };
    res
}

fn check_to(p1: Point, p2: Point, to_check: &[Point]) -> bool {
    let bounds = BoundingBox::new(p1, p2);

    let (_next, wrapping_iter) = get_next(to_check.iter().chain(to_check.iter())).unwrap();

    to_check
        .iter()
        .zip(wrapping_iter)
        .all(|(curr, next)| check_points(curr, next, &bounds))
}

fn get_points(file: &str) -> Vec<Point> {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse::<usize>().unwrap();
            let y = iter.next().unwrap().parse::<usize>().unwrap();
            Point { x, y }
        })
        .collect::<Vec<_>>()
}

fn part_two(file: &str) -> ((Point, Point), usize) {
    let points = get_points(file);

    let ranges = cartesian_ranges(points.len());
    let init = ((Point::new(0, 0), Point::new(0, 0)), 0);
    let (p, max) = ranges.fold(init, |(p, max), r| {
        let p1 = points[r.start];
        let p2 = points[r.end];
        let area = (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1);

        if area <= max {
            return (p, max);
        }

        if r.end - r.start == 1 {
            return (p, max);
        }

        let p1 = points[r.start];
        let p2 = points[r.end];

        if check_to(p1, p2, &points) {
            ((points[r.start], points[r.end]), area)
        } else {
            (p, max)
        }
    });
    dbg!(p);
    (p, max)
}

pub fn day_nine() {
    let now = Instant::now();
    let res = part_one();
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
    println!("{res}");

    let now = Instant::now();
    let res = part_two("./input/nine/big.txt");
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
    println!("{}", res.1);

    // let points = get_points("./input/nine/big.txt");
    // let max_dim = points.iter().fold(Point::new(0, 0), |p, curr| {
    //     Point::new(max(p.x, curr.x), max(p.y, curr.y))
    // });

    // let div = 100;
    // let mut backend = BitMapBackend::new(
    //     "test.png",
    //     ((max_dim.x / div) as u32, (max_dim.y / div) as u32),
    // );

    // let pointos = points
    //     .iter()
    //     .map(|p| ((p.x / div) as i32, (p.y / div) as i32))
    //     .collect_vec();
    // backend.draw_path(pointos, &RED).unwrap();

    // let bounds = BoundingBox::new(res.0.0, res.0.1);
    // backend
    //     .draw_rect(
    //         ((bounds.x_min / div) as i32, (bounds.y_min / div) as i32),
    //         (
    //             (bounds.x_max / div) as i32 + 3,
    //             (bounds.y_max / div) as i32 + 3,
    //         ),
    //         &GREEN,
    //         true,
    //     )
    //     .unwrap();

    // backend.present().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_points() {
        let b = BoundingBox::new(Point::new(2, 5), Point::new(7, 3));
        assert!(!check_points(&Point::new(2, 5), &Point::new(7, 5), &b));
        assert!(check_points(&Point::new(2, 5), &Point::new(2, 3), &b));
        assert!(!check_points(&Point::new(4, 0), &Point::new(4, 8), &b));
    }

    #[test]
    fn test_check_to() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(2, 0);
        let p3 = Point::new(2, 2);
        let p4 = Point::new(0, 2);

        let points = [p1, p2, p3, p4];

        assert!(check_to(points[0], points[2], &points));

        let p1 = Point::new(0, 0);
        let p2 = Point::new(4, 0);
        let p3 = Point::new(4, 4);
        let p4 = Point::new(3, 4);
        let p5 = Point::new(3, 3);
        let p6 = Point::new(0, 3);

        let points = [p1, p2, p3, p4, p5, p6];

        assert!(check_to(points[0], points[5], &points));
        assert!(check_to(points[0], points[1], &points));
        assert!(!check_to(points[0], points[2], &points));

        println!("succeeded");

        let points = get_points("./input/nine/small.txt");
        assert!(check_to(points[4], points[6], &points));

        let points = get_points("./input/nine/test.txt");
        assert!(!check_to(points[0], points[9], &points));

        let points = get_points("./input/nine/test.txt");
        assert!(check_to(points[7], points[9], &points));
    }
}
