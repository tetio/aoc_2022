use std::fs;
use day_4::{part_one, part_two};


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let res1 = part_one(input.as_str());
    println!("part one is {}", res1);

    let res1 = part_two(input.as_str());
    println!("part two is {}", res1);
}
