use std::collections::HashMap;

use crate::Move::*;

fn main() {
    let moves = parse(include_str!("../input.txt"));
    //part 1
    let res = count_tail_pos(&moves);
    println!("Summary: {}", res);
    //part 2
    let res = count_new_tail(&moves);
    println!("Summary2: {}", res);
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

fn count_new_tail(moves: &[(Move, u8)]) -> usize {
    let mut pos = [Pos::default(); 10];
    let mut tailpos: HashMap<Pos, ()> = HashMap::new();
    moves.iter().for_each(|(mov, count)| {
        (0..*count).for_each(|_| {
            simulate_new_move(mov.clone(), &mut pos);
            //println!("After move {mov:?}: {pos:?}");
            tailpos.insert(pos[9], ());
        });
    });
    tailpos.len()
}

fn simulate_new_move(mov: Move, positions: &mut [Pos; 10]) {
    let (xdir, ydir) = match mov {
        Up => (0, -1),
        Down => (0, 1),
        Left => (-1, 0),
        Right => (1, 0),
    };
    positions[0].x += xdir;
    positions[0].y += ydir;
    (0..positions.len())
        .collect::<Vec<usize>>()
        .windows(2)
        .for_each(|s| {
            let (hi, ti) = (s[0], s[1]);
            if !is_adjacent(&positions[hi], &positions[ti]) {
                let xdiff = positions[hi].x - positions[ti].x;
                let ydiff = positions[hi].y - positions[ti].y;
                positions[ti].x += xdiff.signum();
                positions[ti].y += ydiff.signum();
            }
        })
}

#[test]
fn test() {
    let moves = parse(include_str!("../sample.txt"));
    //part 1
    let res = count_tail_pos(&moves);
    assert_eq!(res, 13);
    //part 2
    let res = count_new_tail(&moves);
    assert_eq!(res, 1);
    let moves = parse(
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
    );
    let res = count_new_tail(&moves);
    assert_eq!(res, 36);
}
