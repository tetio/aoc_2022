use day_10::{part_one, part_two};
use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found.");
    let res1 = part_one(&input);
    println!("Part one is {}", res1);
    let res2 = part_two(&input);
    println!("Part two is {}", res2);
}
