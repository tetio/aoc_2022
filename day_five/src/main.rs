use std::fs;

use day_five::{part_one, part_two};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let res1 = part_one(&input);
    println!("Part one is {}", res1);

    let res2 = part_two(&input);
    println!("Part two is {}", res2);
}
