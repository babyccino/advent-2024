use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use itertools::Itertools;
use plotters::prelude::BitMapBackend;
use plotters::prelude::*;

use crate::util::{BoundingBox, CardinalDirection, Point, cartesian_ranges, get_next};

type Point2 = Point<u32>;

fn part_one(points: &[Point2]) -> u64 {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| points[i + 1..].iter().map(move |p2| area(*p1, *p2)))
        .max()
        .unwrap()
}

fn check_points(curr: &Point2, next: &Point2, bounds: &BoundingBox<u32>) -> bool {
    if bounds.inside(curr) {
        return false;
    }

    // not necessary
    // let vec = curr.to(*next);
    // let dir = vec.is_cardinal().unwrap();
    // if curr.eq(&bounds.tl()) {
    //     return dir != CardinalDirection::Down;
    // }
    // if curr.eq(&bounds.bl()) {
    //     return dir != CardinalDirection::Right;
    // }
    // if curr.eq(&bounds.tr()) {
    //     return dir != CardinalDirection::Left;
    // }
    // if curr.eq(&bounds.br()) {
    //     return dir != CardinalDirection::Up;
    // }

    let in_y = bounds.inside_y(curr);
    let in_x = bounds.inside_x(curr);

    // i think this would be a nice optimisation if the shapes were more complex
    // if curr.x == bounds.x_min && in_y {
    //     dir != CardinalDirection::Right && dir != CardinalDirection::Down
    // } else if curr.x == bounds.x_max && in_y {
    //     dir != CardinalDirection::Left && dir != CardinalDirection::Up
    // } else if curr.y == bounds.y_min && in_x {
    //     dir != CardinalDirection::Down && dir != CardinalDirection::Left
    // } else if curr.y == bounds.y_max && in_x {
    //     dir != CardinalDirection::Up && dir != CardinalDirection::Right
    // } else

    if in_y
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
    }
}

fn check_to(p1: Point2, p2: Point2, to_check: &[Point2]) -> bool {
    let bounds = BoundingBox::new(&p1, &p2);

    let (_next, wrapping_iter) = get_next(to_check.iter().chain(to_check.iter())).unwrap();

    to_check
        .iter()
        .zip(wrapping_iter)
        .all(|(curr, next)| check_points(curr, next, &bounds))
}

fn get_points(file: &str) -> Vec<Point2> {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse::<u32>().unwrap();
            let y = iter.next().unwrap().parse::<u32>().unwrap();
            Point2 { x, y }
        })
        .collect::<Vec<_>>()
}

fn area(p1: Point2, p2: Point2) -> u64 {
    ((p1.x.abs_diff(p2.x) + 1) as u64)
        .checked_mul((p1.y.abs_diff(p2.y) + 1) as u64)
        .unwrap()
}

type Ans = ((Point2, Point2), u64);

fn op((points_max, max): Ans, p1: Point2, p2: Point2, points: &[Point2]) -> Ans {
    let area = area(p1, p2);

    if area <= max {
        return (points_max, max);
    }

    if check_to(p1, p2, points) {
        ((p1, p2), area)
    } else {
        (points_max, max)
    }
}

fn rec((mut points_max, mut max): Ans, points: &[Point2], slice: &[Point2]) -> Ans {
    let p1 = slice[0];
    let p2 = slice[1];
    let area = area(p1, p2);

    if area > max {
        points_max = (p1, p2);
        max = area;
    }

    if slice.len() <= 2 {
        return (points_max, max);
    }

    if slice.len() >= 3 {
        (points_max, max) = slice
            .iter()
            .fold((points_max, max), |hi, point| op(hi, p1, *point, points));
    }

    rec((points_max, max), points, &slice[1..])
}

fn part_two(points: &[Point2]) -> Ans {
    let init = ((Point2::new(0, 0), Point2::new(0, 0)), 0);

    let slice = &points[..];
    let (p, max) = rec(init, &points[..], slice);
    (p, max)
}

pub fn day_nine() {
    let now = Instant::now();
    let points = get_points("./input/nine/small.txt");
    let res = part_one(&points);
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
    println!("{res}");

    let now = Instant::now();
    let res = part_two(&points);
    assert_eq!(res.1, 24);
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
    println!("{}", res.1);

    // display(&points, res);

    let now = Instant::now();
    let points = get_points("./input/nine/big.txt");
    let res = part_one(&points);
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
    println!("{res}");

    let now = Instant::now();
    let res = part_two(&points);
    assert_eq!(res.1, 1637556834);
    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
    println!("{}", res.1);

    // display(&points, res);
}

fn display(points: &[Point2], (max_points, _): Ans) {
    let max_dim = points.iter().fold(Point2::new(0, 0), |p, curr| {
        let max_x = max(p.x, curr.x);
        let max_y = max(p.y, curr.y);
        Point2::new(max_x, max_y)
    });

    let scale_down = 100;
    let max_dim_x = (max_dim.x / scale_down) as u32;
    let max_dim_y = (max_dim.y / scale_down) as u32;
    let mut backend = BitMapBackend::new("test.png", (max_dim_x, max_dim_y));

    let pointos = points
        .iter()
        .map(|p| ((p.x / scale_down) as i32, (p.y / scale_down) as i32));
    backend.draw_path(pointos, &RED).unwrap();

    let bounds = BoundingBox::new(&max_points.0, &max_points.1);

    let x_min = (bounds.x_min / scale_down) as i32;
    let y_min = (bounds.y_min / scale_down) as i32;
    let x_max = (bounds.x_max / scale_down) as i32 + 3;
    let y_max = (bounds.y_max / scale_down) as i32 + 3;
    backend
        .draw_rect((x_min, y_min), (x_max, y_max), &GREEN, true)
        .unwrap();

    backend.present().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_points() {
        let b = BoundingBox::new(&Point2::new(2, 5), &Point2::new(7, 3));
        assert!(!check_points(&Point2::new(2, 5), &Point2::new(7, 5), &b));
        assert!(check_points(&Point2::new(2, 5), &Point2::new(2, 3), &b));
        assert!(!check_points(&Point2::new(4, 0), &Point2::new(4, 8), &b));
    }

    #[test]
    fn test_check_to() {
        let p1 = Point2::new(0, 0);
        let p2 = Point2::new(2, 0);
        let p3 = Point2::new(2, 2);
        let p4 = Point2::new(0, 2);

        let points = [p1, p2, p3, p4];

        assert!(check_to(points[0], points[2], &points));

        let p1 = Point2::new(0, 0);
        let p2 = Point2::new(4, 0);
        let p3 = Point2::new(4, 4);
        let p4 = Point2::new(3, 4);
        let p5 = Point2::new(3, 3);
        let p6 = Point2::new(0, 3);

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
