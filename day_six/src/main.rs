use std::fs;
use day_six::{part_one, part_two}; 

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res1 = part_one(&input);
    let res2 = part_two(&input);

    println!("Part one is {}", res1);
    println!("Part two is {}", res2);

}
