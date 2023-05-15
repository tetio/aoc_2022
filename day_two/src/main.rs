use std::fs;

use elf_rock_paper_scissors::{part_one, part_two};


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Result p1 is {}", part_one(&input).to_string());
    println!("Result p2 is {}", part_two(&input).to_string());
}