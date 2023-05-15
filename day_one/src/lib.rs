pub fn part_one(input: &str) -> String {
    let res = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .split("\n")
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    res.to_string()
}

pub fn part_two(input: &str) -> u32 {
    let mut res:Vec<u32> = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .split("\n")
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    res.sort_by(|a, b| b.cmp(a));
    res.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    #[test]
    fn it_works_part_one() {
        // let a: Vec<&str> =  INPUT.split("\n\n").collect();
        // let b: Vec<&str> = a[0].split("\n").collect();
        let res = part_one(INPUT); 
        assert_eq!("24000", res)
    }

    #[test]
    fn it_works_part_two() {
        // let a: Vec<&str> =  INPUT.split("\n\n").collect();
        // let b: Vec<&str> = a[0].split("\n").collect();
        let res = part_two(INPUT); 
        assert_eq!(45000, res)
    }

}