use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::{BufReader, prelude::BufRead};
use std::{thread, time};

use crate::util::{Point, double_iter, moore};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum Mask {
    Empty = 0x0,
    Paper = 0x1,
    Access = 0x11,
}

impl Mask {
    fn has_paper(self) -> bool {
        self as u8 != 0
    }
    fn char(&self) -> char {
        match self {
            Self::Empty => ' ',
            Self::Paper => '\u{23F9}',
            Self::Access => 'x',
        }
    }
}

type Point2 = Point<usize>;

struct Arr {
    pub dim: Point2,
    data: Vec<Mask>,
}

impl Arr {
    fn new(dim: Point2, data: Vec<Mask>) -> Self {
        Arr { dim, data }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.dim.x
    }

    fn get_at(&self, x: usize, y: usize) -> Mask {
        self.data[self.index(x, y)]
    }
    fn get_at_point(&self, point: Point2) -> Mask {
        self.data[self.index(point.x, point.y)]
    }

    fn paper_around(&self, x: usize, y: usize) -> u32 {
        moore(Point2 { x, y }, self.dim).fold(0, |total, point| {
            self.get_at_point(point).has_paper() as u32 + total
        })
    }

    fn can_place_at(&self, x: usize, y: usize) -> bool {
        let res = self.paper_around(x, y);
        res < 4
    }

    fn access_total(&self) -> u32 {
        double_iter(0..self.dim.x, 0..self.dim.y).fold(0, |total, (x, y)| {
            total + (self.get_at(x, y).has_paper() && self.can_place_at(x, y)) as u32
        })
    }

    fn access_total_removing(self) -> u32 {
        self.access_total_removing_inner(0)
    }
    fn access_total_removing_inner(mut self, total: u32) -> u32 {
        let curr = double_iter(0..self.dim.x, 0..self.dim.y).fold(0, |total, (x, y)| {
            let res = self.get_at(x, y).has_paper() && self.can_place_at(x, y);
            if res {
                let index = self.index(x, y);
                self.data[index] = Mask::Access;
            }
            total + res as u32
        });

        if curr == 0 {
            return total;
        }

        for el in self.data.iter_mut() {
            *el = match *el {
                Mask::Access | Mask::Empty => Mask::Empty,
                Mask::Paper => Mask::Paper,
            }
        }

        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);

        self.access_total_removing_inner(total + curr)
    }
}

impl Display for Arr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut str = String::new();
        str.reserve(self.dim.x * self.dim.y + self.dim.y);
        let str = self.data.iter().enumerate().fold(str, |mut str, (i, m)| {
            str.push(m.char());
            str.push(m.char());
            if (i + 1) % self.dim.y == 0 {
                str.push('\n');
            }

            str
        });
        write!(f, "{}", str)
    }
}

fn get_arr(loc: &str) -> Arr {
    let file = File::open(loc).unwrap();
    let reader = BufReader::new(file);

    let init = (Point::new(0, 0), Vec::new());
    let (dim, data) = reader
        .lines()
        .map(|line| line.unwrap())
        .fold(init, |(dim, data), line| {
            let mapped = line.chars().map(|c| match c {
                '@' => Mask::Paper,
                _ => Mask::Empty,
            });
            let joined = data.into_iter().chain(mapped).collect::<Vec<_>>();
            let dim = Point {
                x: line.len(),
                y: dim.y + 1,
            };
            (dim, joined)
        });

    Arr::new(dim, data)
}

pub fn part_one() -> u32 {
    let arr = get_arr("./input/four/big.txt");
    arr.access_total()
}

pub fn part_two() -> u32 {
    let arr = get_arr("./input/four/big.txt");
    arr.access_total_removing()
}
