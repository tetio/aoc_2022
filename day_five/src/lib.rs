use std::str;
use std::iter;


pub fn part_one(input: &String) -> String {
    let data = input.split("\n\n").map(|e| e.to_string()).collect::<Vec<_>>();
    let stacks = make_stacks(&data[0]);
    let moves = make_moves(&data[1]);
    let res =apply_moves(stacks, moves);
    res.to_owned()
}

fn make_stacks(input: &str) -> Vec<Vec<String>>{
    let data = input.split("\n").map(|e| format!("{}{}", e, " ")).collect::<Vec<_>>();

    let num_stacks = data[data.len()-1].len() / 4;
    let bottom_row = data.len()-2;
    let mut stacks: Vec<_> =iter::repeat_with(|| Vec::new())
        .take(num_stacks)
        .collect();
    for i in (0..=bottom_row).rev() {
        let row = &data[i];
        let crates = row.as_bytes().chunks(4)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();
        crates.iter().enumerate()
            .filter(|&(_, e)| e.trim().len() > 0)
            .for_each(|(i, e)| {
                let a_new_crate: &str = (*e).clone();
                stacks[i].push(a_new_crate.to_owned());
            });
    }

    stacks
}

fn make_moves(input: &str) -> Vec<(i32, usize, usize)> {
    let moves = input.split("\n").into_iter()
        .map(|e| {
            let a_move = e.split("from").collect::<Vec<_>>();
            let times = a_move[0].split("move").collect::<Vec<_>>()[1].trim();
            let aux = a_move[1]
            .split("to").map(|e| e.trim()).collect::<Vec<_>>();
            (times.parse::<i32>().unwrap(), aux[0].parse::<usize>().unwrap()-1, aux[1].parse::<usize>().unwrap()-1)
        })
        .collect::<Vec<_>>();

        moves
}

fn apply_moves(is: Vec<Vec<String>>, moves: Vec<(i32, usize, usize)>) -> String {
    let mut stacks = is.clone();
    moves.iter().for_each(|(i, o, d)| {
        for _ in (0..*i) {
            let oc = stacks[*o].pop().unwrap();
            stacks[*d].push(oc);
    
        }
    });
    let res = stacks.iter()
        .map(|stack| 
            if stack.len()>0 {
                stack.last().unwrap().trim().chars().nth(1).unwrap()
            } else {' '})
        .collect::<Vec<_>>();   
    res.iter().collect()
}

fn apply_moves_2(is: Vec<Vec<String>>, moves: Vec<(i32, usize, usize)>) -> String {
    let mut stacks = is.clone();
    moves.iter().for_each(|(i, o, d)| {
        let idx = stacks[*o].len()  - *i as usize;
        let mut to_move = stacks[*o].split_off(idx);//split_array_ref::<idx>();
        stacks[*d].append(&mut to_move);
    });
    let res = stacks.iter() 
        .map(|stack| if stack.len()>0 {
            let a_crate =  stack.last().unwrap().trim();
            a_crate.chars().nth(1).unwrap()
        } else {' '})
        .collect::<Vec<_>>();
    res.iter().collect()
}

pub fn part_two(input: &String) -> String {
    let data = input.split("\n\n").map(|e| e.to_string()).collect::<Vec<_>>();
    let stacks = make_stacks(&data[0]);
    let moves = make_moves(&data[1]);
    let res =apply_moves_2(stacks, moves);
    res.to_owned()
}




#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
  1  2   3  

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    #[test] 
    fn part_one_001() {
        let res = part_one(&INPUT.to_string());
        assert_eq!("CMZ", res);
    }
    #[test] 
    fn part_two_001() {
        let res = part_two(&INPUT.to_string());
        assert_eq!("MCD", res);
    }
}