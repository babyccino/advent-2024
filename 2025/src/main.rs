mod five;
mod four;
mod one;
mod three;
mod two;
pub mod util;

use crate::five::{part_one, part_two};

fn main() {
    println!("day one, p1: {}", part_one());
    println!("day one, p2: {}", part_two());
}
