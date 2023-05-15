use std::collections::HashMap;


pub fn part_one(input: &str) -> u32 {
    let res = input.split("\n")
        .map(|elem| {
            let mut raw = elem.split_ascii_whitespace();
            check_score((raw.next().unwrap(), raw.next().unwrap()))
        }).sum::<u32>();
        //.collect::<Vec<_>>();
    res
}

pub fn part_two(input: &str) -> u32 {
    let res = input.split("\n")
        .map(|elem| {
            let mut raw = elem.split_ascii_whitespace();
            check_score_two((raw.next().unwrap(), raw.next().unwrap()))
        }).sum::<u32>();
        //.collect::<Vec<_>>();
    res
}

#[derive(PartialEq, Clone, Copy)]
enum Game {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3
}

fn points_per_move(ddd: &Game) -> u32 {
    let mut res = 3;
    if *ddd == Game::ROCK {
        res = 1
    } else if *ddd == Game::PAPER {
        res = 2
    } 
    res
}

fn check_score((hers, mine) : (&str, &str)) -> u32 {
    let scores: HashMap<&str, Game> = HashMap::from([
    ("A",Game::ROCK),
    ("X",Game::ROCK),
    ("B",Game::PAPER),
    ("Y",Game::PAPER),
    ("C",Game::SCISSORS),
    ("Z",Game::SCISSORS),
    ]);
    let her_move = scores[hers];
    let my_move = scores[mine];
    let mut res = points_per_move(&my_move);
    if her_move == my_move {
        res += 3;
    } else if (her_move == Game::PAPER && my_move == Game::SCISSORS) 
        || (her_move == Game::ROCK && my_move == Game::PAPER)
        || (her_move == Game::SCISSORS && my_move == Game::ROCK) {
        res += 6;
    } else if (her_move == Game::PAPER && my_move == Game::ROCK)
        || (her_move == Game::ROCK && my_move == Game::SCISSORS)
        || (her_move == Game::SCISSORS && my_move == Game::PAPER) {
        res += 0;
    }
    res
}

fn check_score_two((hers, mine) : (&str, &str)) -> u32 {
    let score =  match mine {
        "X" => lose(&hers),
        "Y" => draw(&hers),
        _ => win(&hers)
    };
    score
}


fn lose(mv: &str) -> u32 {
    match mv {
        "A" => 3,
        "B" => 1,
        _ => 2
    }
}

fn draw(mv: &str) -> u32 {
   let score = match mv {
        "A" => 1,
        "B" => 2,
        _ => 3
    };
    score + 3
}

fn win(mv: &str) -> u32 {
    let score = match mv {
        "A" => 2,
        "B" => 3,
        _ => 1
    };
    score + 6
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";
    #[test]
    fn test_part_one() {
        let res = part_one(INPUT);   
        assert_eq!(15, res);    
    } 
    #[test]
    fn test_part_two() {
        let res = part_two(INPUT);   
        assert_eq!(12, res);    
    } 
}