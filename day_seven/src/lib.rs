use std::collections::HashMap;

#[derive(Clone, Debug)]
struct State {
    pub map: HashMap<String, i32>,
    pub current_dir: String,
}

pub fn part_one(input: &String) -> i32 {
    let mut state = State {map: HashMap::new(), current_dir: "".to_string()};

    input.split("\n").into_iter()
        .for_each(|line| 
            match line.starts_with("$") {
                true => state = eval_command(&line, &state),
                false => state = eval_data(line, &state),
            }
        );
    0
}

fn eval_command(line: &str, state: &State) -> State{
    let command = line.split(" ").collect::<Vec<_>>();
    match command[0] {
        "$cd" => do_change_dir(command[1], &state),
        "$ls" => state,
    }
}

fn do_change_dir(dir: &str, state: &State) -> State {
    let mut current_dir = &state.current_dir;
    current_dir = match dir {
        ".." =>   {
            let last = current_dir.rfind('/').unwrap();
            &current_dir.get(0..last).unwrap().to_string()
        },
        _ => current_dir + "/" + dir
    }

    let mut new_state = state.clone();
    new_state.current_dir = current_dir:

    new_state
}
fn eval_data(line: &str, state: &State) -> State{
    let mut new_state = state.clone();
    let data = line.split(" ");
    match data[0] {
        "dir" => 
    }
}


pub fn part_two(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part_one_001() {
        let res1 = part_one(&INPUT.to_string());
        assert_eq!(12, res1)
    }
}