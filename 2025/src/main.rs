mod four;
mod one;
mod three;
mod two;
pub mod util;

use crate::four::{part_one, part_two};
use crate::util::{double_iter, moore, Point};

fn main() {
    println!("day one, p1: {}", part_one());
    println!("day one, p2: {}", part_two());
    // for hi in moore(Point::new(0, 0), Point::new(10, 10)) {
    //     dbg!(hi);
    // }
}
