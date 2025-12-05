mod five;
mod four;
mod one;
mod three;
mod two;
pub mod util;

use crate::five::{day_five, get_data, part_one, part_two};

fn main() {
    let (ranges, data) = get_data("./input/five/big.txt");
    let total_runs = 10000000;
    let (part_one, part_two, total) = (0..total_runs).fold((0, 0, 0), |(_, _, total), i| {
        let part_one = part_one(&ranges, &data);
        let part_two = part_two(&ranges);
        if i % (total_runs / 10) == 0 {
            println!("10% done");
        }
        (part_one, part_two, part_one + part_two + total)
    });
    println!("day one, p1: {} should be 862", part_one);
    println!("day one, p1: {} should be 357907198933892", part_two);
    println!("{total}");
}
