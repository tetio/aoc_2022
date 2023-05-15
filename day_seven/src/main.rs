use std::fs;

use day_seven::{part_one, part_two};
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res1 = part_one(&input);
    let res2 = part_two(&input);

    println!("Res1 is {}", res1);
    println!("Res2 is {}", res2)
}
