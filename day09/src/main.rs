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
    let mut count = 0;
    for _ in moves.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
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
