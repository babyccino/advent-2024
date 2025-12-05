mod five;
mod four;
mod one;
mod three;
mod two;
pub mod util;

use crate::two::{part_one, part_two};

fn main() {
    for _ in 0..10 {
        println!("day one, p1: {} should be 13108371860", part_one());
        println!("day one, p2: {} shoudl be 22471660255", part_two());
    }
}
