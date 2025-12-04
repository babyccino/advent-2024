use std::fs::read_to_string;

fn check_num(num: i64) -> i64 {
    let str = num.to_string();
    if str.len() % 2 == 0 {
        let first = &str[..str.len() / 2];
        let second = &str[str.len() / 2..];
        if first == second {
            num
        } else {
            0
        }
    } else {
        0
    }
}

fn check_range(slice: &str) -> i64 {
    let dash = slice.find('-').unwrap();
    let start_slice = &slice[..dash];
    let Ok(start) = start_slice.parse::<i64>() else {
        panic!("failed parsing {start_slice}");
    };
    let end_slice = &slice[(dash + 1)..];
    let Ok(end) = end_slice.parse::<i64>() else {
        panic!("failed parsing {end_slice}");
    };
    (start..=end).fold(0, |total, num| total + check_num(num))
}

pub fn part_one() -> i64 {
    let file = read_to_string("./input/two/big.txt").unwrap();

    let mut result = 0;
    let mut slice = &file[..];

    loop {
        let ptr = slice.find(',').unwrap_or(slice.len());

        result += check_range(&slice[..ptr].trim());

        if ptr == slice.len() {
            return result;
        }
        slice = &slice[(ptr + 1)..];
        if slice.is_empty() {
            return result;
        }
    }
}

fn check_num_two(num: i64) -> i64 {
    // println!("\nchecking: {num}");
    let str = num.to_string();
    let mut div = 1;

    while div <= str.len() / 2 {
        // println!("div: {div}");
        if str.len() % div != 0 {
            div = div + 1;
            continue;
        }

        let slice = &str[..div];
        // println!("slice: {slice}");

        let mut correct = true;
        for i in 1..(str.len() / div) {
            let check = &str[i * div..(i + 1) * div];

            if slice != check {
                correct = false;
                break;
            }
        }

        if correct {
            // println!("{num}, {div}, yes");
            return num;
        }

        div = div + 1;
    }

    0
}

fn check_range_two(slice: &str) -> i64 {
    let dash = slice.find('-').unwrap();

    let end = slice[(dash + 1)..].parse::<i64>().unwrap();
    if end < 11 {
        return 0;
    }

    let start = slice[..dash].parse::<i64>().unwrap();
    let start = if start < 11 { 11 } else { start };

    (start..end).fold(0, |total, num| total + check_num_two(num))
}

pub fn part_two() -> i64 {
    let file = read_to_string("./input/two/big.txt").unwrap();

    let mut result = 0;
    let mut slice = &file[..];

    loop {
        let ptr = slice.find(',').unwrap_or(slice.len());

        result += check_range_two(&slice[..ptr].trim());

        if ptr == slice.len() {
            return result;
        }
        slice = &slice[(ptr + 1)..];
        if slice.is_empty() {
            return result;
        }
    }
}
