pub fn part_one(data: &str) -> u32 {
    let res = data.split("\n")
        .map(|l| eval_racksack(l.split_at(l.len()/2)))
        .sum();
        res
}


fn eval_racksack((left, right): (&str, &str)) -> u32 {
    let items: Vec<_> = left.chars().filter(|c| right.contains(*c)).collect();
    let p = items.get(0).unwrap();
    calculate_priority(p)
}

fn calculate_priority(p: &char) -> u32 {
    let res = if p.is_lowercase() {*p as u32 - 96} else {*p as u32 - 64 + 26};
    res
} 

fn find_common(r1: &str, r2: &str, r3: &str) -> u32 {
    let res0 = r1.chars().filter(|c| r2.contains(*c)).filter(|c| r3.contains(*c)).collect::<Vec<_>>();
    calculate_priority(&res0[0])
}

pub fn part_two(data: &str) -> u32 {
    let lines: Vec<_> = data.split("\n").collect();
    let mut res: Vec<u32> = Vec::new();
    for i in (0..lines.len()).step_by(3) {
        res.push(find_common(lines[i],lines[i+1], lines[i+2]));
    }
    res.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
const INPUT_2_1: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";
const INPUT_2_2: &str = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    #[test]
    fn part_one_001() {
        let res = part_one(INPUT);
        assert_eq!(157, res);
    }
    #[test]
    fn part_two_001() {
        let res = part_two(INPUT_2_1);
        assert_eq!(18, res);
    }
    #[test]
    fn part_two_002f() {
        let res = part_two(INPUT_2_2);
        assert_eq!(52, res);
    }
}