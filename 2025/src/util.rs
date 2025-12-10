use std::{
    cmp::{max, min},
    fmt::{self, Debug, Formatter},
    iter,
    ops::{Add, Range, Rem},
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn to(self, other: Self) -> Vector {
        Vector {
            x: other.x as isize - self.x as isize,
            y: other.y as isize - self.y as isize,
        }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

impl Vector {
    pub fn is_cardinal(self) -> Option<CardinalDirection> {
        match self {
            Vector { x, y } if x > 0 && y == 0 => Some(CardinalDirection::Right),
            Vector { x, y } if x == 0 && y > 0 => Some(CardinalDirection::Down),
            Vector { x, y } if x < 0 && y == 0 => Some(CardinalDirection::Left),
            Vector { x, y } if x == 0 && y < 0 => Some(CardinalDirection::Up),
            _ => None,
        }
    }

    pub fn cardinal(self) -> Vector {
        match self.is_cardinal().unwrap() {
            CardinalDirection::Right => Vector { x: 1, y: 0 },
            CardinalDirection::Down => Vector { x: 0, y: 1 },
            CardinalDirection::Left => Vector { x: -1, y: 0 },
            CardinalDirection::Up => Vector { x: 0, y: -1 },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub x_min: usize,
    pub x_max: usize,
    pub y_min: usize,
    pub y_max: usize,
}

impl BoundingBox {
    pub fn new(p1: Point, p2: Point) -> BoundingBox {
        let l = min(p1.x, p2.x);
        let r = max(p1.x, p2.x);
        let b = max(p1.y, p2.y);
        let t = min(p1.y, p2.y);

        Self {
            x_min: l,
            x_max: r,
            y_min: t,
            y_max: b,
        }
    }

    pub fn inside(&self, p: Point) -> bool {
        self.inside_x(p) && self.inside_y(p)
    }

    pub fn inside_x(&self, p: Point) -> bool {
        p.x > self.x_min && p.x < self.x_max
    }

    pub fn inside_y(&self, p: Point) -> bool {
        p.y > self.y_min && p.y < self.y_max
    }

    pub fn tl(&self) -> Point {
        Point {
            x: self.x_min,
            y: self.y_min,
        }
    }
    pub fn bl(&self) -> Point {
        Point {
            x: self.x_min,
            y: self.y_max,
        }
    }
    pub fn tr(&self) -> Point {
        Point {
            x: self.x_max,
            y: self.y_min,
        }
    }
    pub fn br(&self) -> Point {
        Point {
            x: self.x_max,
            y: self.y_max,
        }
    }
}

#[derive(Copy, Clone, Hash)]
pub struct Point3d {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Point3d {
    pub fn dist2(&self, other: &Point3d) -> usize {
        let x_sum = self.x as isize - other.x as isize;
        let y_sum = self.y as isize - other.y as isize;
        let z_sum = self.z as isize - other.z as isize;

        (x_sum * x_sum + y_sum * y_sum + z_sum * z_sum) as usize
    }
}

impl fmt::Debug for Point3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Eq for Point3d {}

impl PartialEq for Point3d {
    fn eq(&self, other: &Point3d) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CardinalDirection {
    Up,
    Right,
    Down,
    Left,
}

impl CardinalDirection {
    pub fn flip(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }

    pub fn clockwise(self, other: CardinalDirection) -> bool {
        return if self == Self::Left && other == Self::Up {
            true
        } else {
            other.num() > self.num()
        };
    }
    fn num(self) -> u8 {
        match self {
            Self::Up => 0,
            Self::Down => 1,
            Self::Right => 2,
            Self::Left => 3,
        }
    }
}

pub fn double_iter<I: IntoIterator, J: IntoIterator + Clone>(
    i_iter: I,
    j_iter: J,
) -> impl Iterator<Item = (I::Item, J::Item)>
where
    I::Item: Copy,
{
    i_iter
        .into_iter()
        .flat_map(move |i| j_iter.clone().into_iter().map(move |j| (i, j)))
}

const DIRS: [Vector; 8] = [
    Vector { x: -1, y: -1 },
    Vector { x: 0, y: -1 },
    Vector { x: 1, y: -1 },
    Vector { x: -1, y: 0 },
    Vector { x: 1, y: 0 },
    Vector { x: -1, y: 1 },
    Vector { x: 0, y: 1 },
    Vector { x: 1, y: 1 },
];

pub fn moore(pos: Point, dim: Point) -> impl Iterator<Item = Point> {
    DIRS.iter().filter_map(move |Vector { x, y }| {
        let x_delta = pos.x as isize + *x as isize;
        let y_delta = pos.y as isize + *y as isize;
        if x_delta >= 0 && x_delta < dim.x as isize && y_delta >= 0 && y_delta < dim.y as isize {
            Some(Point {
                x: x_delta as usize,
                y: y_delta as usize,
            })
        } else {
            None
        }
    })
}

pub fn real_mod<T: Rem<Output = T> + Add<Output = T> + Copy>(l: T, r: T) -> T {
    (l % r + r) % r
}

pub fn previous_iter<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = Option<T>> {
    iter::once(None).chain(iter.map(|el| Some(el)))
}

pub fn next_iter<T>(mut iter: impl Iterator<Item = T>) -> impl Iterator<Item = Option<T>> {
    _ = iter.next();
    iter.map(|el| Some(el)).chain(iter::once(None))
}

pub fn get_next<T, TIter: Iterator<Item = T>>(mut iter: TIter) -> Option<(T, TIter)> {
    let next = iter.next();
    next.map(|next| (next, iter))
}

fn ranges_overlap(r1: &Range<usize>, r2: &Range<usize>) -> bool {
    let (r1, r2) = if r1.start <= r2.start {
        (r1, r2)
    } else {
        (r2, r1)
    };

    r1.end >= r2.start
}

// 0..1
// 0..2
// ..
// 0..len
// ..
// 1..2
// 1..len
// ..
// len-1..len
pub fn cartesian_ranges(len: usize) -> impl Iterator<Item = Range<usize>> {
    (0..(len - 1)).flat_map(move |i| ((i + 1)..(len)).map(move |j| i..j))
}
