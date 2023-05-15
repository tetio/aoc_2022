
pub fn part_one(input: &str) -> u32 {
    input.lines().map(|pair| process_pair1(pair)).sum()
}

pub fn part_two(input: &str) -> u32 {
    input.lines().map(|pair| process_pair2(pair)).sum()
}


fn process_pair1(input: &str) -> u32 {
    let (first_assigment, second_assigment) = get_assigments(input);  
    check_assigments1(first_assigment, second_assigment)
}


fn process_pair2(input: &str) -> u32 {
    let (first_assigment, second_assigment) = get_assigments(input);  
    check_assigments2(first_assigment, second_assigment)
}

fn get_assigments(input: &str) -> (Vec<u32>, Vec<u32>) {
    let pairs = input.split(",").collect::<Vec<_>>();
    let first_range = make_range(pairs[0]);
    let first_assigment = make_list(first_range.0, first_range.1);
    let second_range = make_range(pairs[1]);
    let second_assigment = make_list(second_range.0, second_range.1);   
    (first_assigment, second_assigment)
}


fn make_range(input: &str) -> (u32, u32) {
    let pos = input.split("-").collect::<Vec<_>>();
    (pos[0].parse::<u32>().unwrap(), pos[1].parse::<u32>().unwrap()) 
}


fn make_list(start: u32, end: u32) -> Vec<u32> {
    let mut res = vec![];
    for i in start..=end {
        res.push(i);
    }
    res
}

fn check_assigments1(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let mut res = 0;
    if (left[0] >= right[0] && left[left.len()-1] <= right[right.len()-1]) ||
        (right[0] >= left[0] && right[right.len()-1] <= left[left.len()-1]) {
            res = 1;
        }
    res
}

fn check_assigments2(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let res = left.iter().filter(|e| right.contains(e)).collect::<Vec<_>>();
    if res.len() > 0 {1} else {0}
}

#[cfg(test)] 
mod tests {
    use crate::{part_one, part_two};
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    #[test]
    fn p1_000() {
        let res = part_one(INPUT);
        assert!(2 == res)
    }

    #[test]
    fn p2_000() {
        let res = part_two(INPUT);
        assert!(4 == res)
    }
}
