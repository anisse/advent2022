use std::collections::HashMap;

use crate::Move::*;

fn main() {
    let moves = parse(include_str!("../input.txt"));
    //part 1
    let res = count_tail_pos(&moves);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&moves);
    //println!("Summary2: {}", res);
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}
fn parse(input: &str) -> Vec<(Move, u8)> {
    input
        .lines()
        .map(|l| {
            let mut s = l.split_ascii_whitespace();
            (
                match s.next().expect("no move") {
                    "U" => Up,
                    "D" => Down,
                    "L" => Left,
                    "R" => Right,
                    _ => panic!("unexpected move"),
                },
                s.next().expect("no count").parse().expect("not int"),
            )
        })
        .collect()
}

fn count_tail_pos(moves: &[(Move, u8)]) -> usize {
    let (mut head, mut tail) = (Pos::default(), Pos::default());
    let mut tailpos: HashMap<Pos, ()> = HashMap::new();
    moves.iter().for_each(|(mov, count)| {
        (0..*count).for_each(|_| {
            (head, tail) = simulate_move(mov.clone(), head, tail);
            tailpos.insert(tail, ());
        });
    });
    tailpos.len()
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Hash, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

fn simulate_move(mov: Move, mut head: Pos, mut tail: Pos) -> (Pos, Pos) {
    let (xdir, ydir) = match mov {
        Up => (0, -1),
        Down => (0, 1),
        Left => (-1, 0),
        Right => (1, 0),
    };
    let headinit = head;
    head.x += xdir;
    head.y += ydir;
    if !is_adjacent(&head, &tail) {
        // tail "catchup"
        tail.x = headinit.x;
        tail.y = headinit.y;
    }
    (head, tail)
}

fn is_adjacent(head: &Pos, tail: &Pos) -> bool {
    let xdiff = head.x - tail.x;
    let ydiff = head.y - tail.y;
    (-1..=1).contains(&xdiff) && (-1..=1).contains(&ydiff)
}

#[test]
fn test() {
    let moves = parse(include_str!("../sample.txt"));
    //part 1
    let res = count_tail_pos(&moves);
    assert_eq!(res, 13);
    //part 2
    // let res = operation2(&moves);
    // assert_eq!(res, 42);
}
