use advent2022::*;

use crate::Tile::*;

fn main() {
    let things = parse(input!());
    //part 1
    let res = operation(&things);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&things);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Map {
    input
        .lines()
        .skip(1)
        .take_while(|l| l.chars().nth(1).unwrap() != '#')
        .map(|l| {
            l.chars()
                .skip(1)
                .take(l.len() - 2)
                .map(|c| match c {
                    '.' => Empty,
                    '>' => Right,
                    '<' => Left,
                    '^' => Up,
                    'v' => Down,
                    _ => panic!("unknown char {c}"),
                })
                .collect()
        })
        .collect()
}
type Map = Vec<Vec<Tile>>;
type MapSlice = [Vec<Tile>];

#[derive(Debug, Clone, Copy)]
enum Tile {
    Up,
    Down,
    Left,
    Right,
    Empty,
}

#[derive(Debug, Clone)]
struct Pos {
    x: u8,
    y: u8,
}

fn operation(things: &MapSlice) -> usize {
    let mut count = 0;
    for _ in things.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = operation(&things);
    assert_eq!(res, 42);
    //part 2
    // let res = operation2(&things);
    // assert_eq!(res, 42);
}
