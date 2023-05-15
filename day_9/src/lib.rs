use std::collections::HashSet;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

pub fn part_one(input: &String) -> i32 {
    let mut positions_visited: HashSet<Position> = HashSet::new();
    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };

    let movements = input.lines().map(|l| l.split(" ").collect::<Vec<&str>>()); //.collect::<Vec<Vec<&str>>>();
    movements.for_each(|mov| {
        let (path, hp, tp) = move_knots(mov, head_position, tail_position);
        head_position = hp;
        tail_position = tp;
        path.iter().for_each(|e| {
            positions_visited.insert(e.clone());
        });
    });
    positions_visited.len() as i32
}

pub fn part_two(input: &String) -> i32 {
    let mut positions_visited: HashSet<Position> = HashSet::new();
    let mut knots = vec![Position { x: 0, y: 0 }; 10];

    let movements = input.lines().map(|l| l.split(" ").collect::<Vec<&str>>()); //.collect::<Vec<Vec<&str>>>();
    movements.for_each(|mov| {
        let (path, new_knots) = move_10_knots(mov, knots.clone());
        knots = new_knots;
        path.iter().for_each(|e| {
            positions_visited.insert(e.clone());
        });
    });
    positions_visited.len() as i32
}

fn move_knots(
    mov: Vec<&str>,
    mut head_pos: Position,
    mut tail_pos: Position,
) -> (Vec<Position>, Position, Position) {
    let mut path: Vec<Position> = Vec::new();
    let dir = mov[0];
    let steps = mov[1].parse::<i32>().unwrap();
    for _ in 0..steps {
        match dir {
            "U" => move_up(&mut head_pos, &mut tail_pos),
            "D" => move_down(&mut head_pos, &mut tail_pos),
            "R" => move_right(&mut head_pos, &mut tail_pos),
            "L" => move_left(&mut head_pos, &mut tail_pos),
            _ => (),
        }
        path.push(tail_pos.clone());
    }
    (path, head_pos, tail_pos)
}

fn move_left(head_pos: &mut Position, tail_pos: &mut Position) {
    head_pos.x -= 1;
    left(head_pos, tail_pos);
}

fn left(head_pos: &mut Position, tail_pos: &mut Position) {
    if !same_column(&*tail_pos, &*head_pos)
        && !same_row(&*tail_pos, &*head_pos)
        && !are_touching(&*tail_pos, &*head_pos)
    {
        tail_pos.x = head_pos.x + 1;
        tail_pos.y = head_pos.y;
    } else if same_row(&*tail_pos, &*head_pos) && !are_touching(&*tail_pos, &*head_pos) {
        tail_pos.x -= 1;
    }
}



fn move_right(head_pos: &mut Position, tail_pos: &mut Position) {
    head_pos.x += 1;
    right(head_pos, tail_pos);
}

fn right(head_pos: &mut Position, tail_pos: &mut Position) {
    if !same_column(&*tail_pos, &*head_pos)
        && !same_row(&*tail_pos, &*head_pos)
        && !are_touching(&*tail_pos, &*head_pos)
    {
        tail_pos.x = head_pos.x - 1;
        tail_pos.y = head_pos.y;
    } else if same_row(&*tail_pos, &*head_pos) && !are_touching(&*tail_pos, &*head_pos) {
        tail_pos.x += 1;
    }
}



fn move_down(head_pos: &mut Position, tail_pos: &mut Position) {
    head_pos.y -= 1;
    down(head_pos, tail_pos);
}

fn down(head_pos: &mut Position, tail_pos: &mut Position) {
    if !same_row(&*tail_pos, &*head_pos)
        && !same_column(&*tail_pos, &*head_pos)
        && !are_touching(&*tail_pos, &*head_pos)
    {
        tail_pos.y = head_pos.y + 1;
        tail_pos.x = head_pos.x;
    } else if same_column(&*tail_pos, &*head_pos) && !are_touching(&*tail_pos, &*head_pos) {
        tail_pos.y -= 1;
    }
}



fn move_up(head_pos: &mut Position, tail_pos: &mut Position) {
    head_pos.y += 1;
    up(head_pos, tail_pos);
}

fn up(head_pos: &mut Position, tail_pos: &mut Position) {
    if !same_row(&*tail_pos, &*head_pos)
        && !same_column(&*tail_pos, &*head_pos)
        && !are_touching(&*tail_pos, &*head_pos)
    {
        tail_pos.y = head_pos.y - 1;
        tail_pos.x = head_pos.x;
    } else if same_column(&*tail_pos, &*head_pos) && !are_touching(&*tail_pos, &*head_pos) {
        tail_pos.y += 1;
    }
}



fn mov1(i: usize, knots: &mut Vec<Position>) -> Position {
    let diffc = knots[i-1].y - knots[i].y;
    let diffr = knots[i-1].x - knots[i].x;

    if diffc == 0 && diffr.abs() > 1 {
        knots[i].x += diffr.signum();
    } else if diffr == 0 && diffc.abs() > 1 {
        knots[i].y += diffc.signum();
    } else if diffc.abs() > 1 || diffr.abs() > 1 {
        knots[i].x += diffr.signum();
        knots[i].y += diffc.signum();
    }
    knots[i].clone()
}

fn move_10_knots(mov: Vec<&str>, mut knots: Vec<Position>) -> (Vec<Position>, Vec<Position>) {
    let mut path: Vec<Position> = Vec::new();
    let dir = mov[0];
    let steps = mov[1].parse::<i32>().unwrap();

    for step in 0..steps {
        match dir {
            "U" => {
                knots[0].y += 1;
                for i in 1..knots.len() {
                    let p = mov1(i, &mut knots); 
                }
                path.push(knots[knots.len()-1]);
            }
            "D" => {
                knots[0].y -= 1;
                for i in 1..knots.len() {
                    let p = mov1(i, &mut knots); 
                }
                path.push(knots[knots.len()-1]);
            }
            "R" => {
                knots[0].x += 1;
                for i in 1..knots.len() {
                    let p = mov1(i, &mut knots); 
                }
                path.push(knots[knots.len()-1]);
            }
            "L" => {
                knots[0].x -= 1;
                for i in 1..knots.len() {
                    let p = mov1(i, &mut knots); 
                }
                path.push(knots[knots.len()-1]);
            }
            _ => (),
        }
    }

    (path, knots)
}

fn same_column(tail_pos: &Position, head_pos: &Position) -> bool {
    tail_pos.x == head_pos.x
}

fn same_row(tail_pos: &Position, head_pos: &Position) -> bool {
    tail_pos.y == head_pos.y
}

fn are_touching(tail_pos: &Position, head_pos: &Position) -> bool {
    i32::abs(tail_pos.x - head_pos.x) <= 1 && i32::abs(tail_pos.y - head_pos.y) <= 1
}

// fn is_too_far(tail_pos: &Position, head_pos: &Position) -> bool {
//     tail_pos.x != head_pos.x && tail_pos.y != head_pos.y
//         && ( i32::abs(tail_pos.x - head_pos.x) > 1
//             || i32::abs(tail_pos.y - head_pos.y) > 1)
// }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part_one_001() {
        let res = part_one(&INPUT1.to_string());
        assert_eq!(13, res);
    }

    #[test]
    fn part_two_001() {
        let res = part_two(&INPUT2.to_string());
        assert_eq!(36, res);
    }
}
