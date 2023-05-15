use std::collections::HashMap;


pub fn part_one(input: &String) -> i32 {
    let mut last_four: HashMap<char, i32> = HashMap::with_capacity(4);
    let mut counter = 0i32;
    let mut chars = input.chars();
    let mut ended = false;
    
    while are_all_different(4, &last_four) && !ended {
        match chars.next() {
            Some(c) => {
                last_four = last_four.iter().filter(|&(_k, v)|  counter - *v < 4).map(|(k,v)| (*k, *v)).collect();
                last_four.insert(c, counter);
                counter += 1;
            },
            None => ended = true,
        }
    }

    counter
}

fn are_all_different(index: usize, map: &HashMap<char, i32>) -> bool {
    map.len() < index
}

pub fn part_two(input: &String) -> i32 {
    let mut last_four: HashMap<char, i32> = HashMap::with_capacity(14);
    let mut counter = 0i32;
    let mut chars = input.chars();
    let mut ended = false;
    
    while are_all_different(14, &last_four) && !ended {
        match chars.next() {
            Some(c) => {
                last_four = last_four.iter().filter(|&(_k, v)|  counter - *v < 14).map(|(k,v)| (*k, *v)).collect();
                last_four.insert(c, counter);
                counter += 1;
            },
            None => ended = true,
        }
    }

    counter
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT_2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT_3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT_4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part_one_001() {
        let res1 = part_one(&INPUT_1.to_string());
        assert_eq!(5, res1)
    }
    #[test]
    fn part_one_002() {
        let res1 = part_one(&INPUT_2.to_string());
        assert_eq!(6, res1)
    }
    #[test]
    fn part_one_003() {
        let res1 = part_one(&INPUT_3.to_string());
        assert_eq!(10, res1)
    }
    #[test]
    fn part_one_004() {
        let res1 = part_one(&INPUT_4.to_string());
        assert_eq!(11, res1)
    }

    #[test]
    fn part_two_001() {
        let res1 = part_two(&INPUT_1.to_string());
        assert_eq!(23, res1);
        let res2 = part_two(&INPUT_2.to_string());
        assert_eq!(23, res2);
        let res3 = part_two(&INPUT_3.to_string());
        assert_eq!(29, res3);
        let res4 = part_two(&INPUT_4.to_string());
        assert_eq!(26, res4);
    }

}
