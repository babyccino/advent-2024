#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
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
