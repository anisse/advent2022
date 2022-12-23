use advent2022::*;

use std::collections::VecDeque;

use crate::Tile::*;

fn main() {
    let mut map = parse(input!());
    //part 1
    let res = spread_rounds(&mut map, 10);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&map);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Empty,
                    '#' => Elf,
                    _ => panic!("unknown char {c}"),
                })
                .collect()
        })
        .collect()
}

type Map = VecDeque<VecDeque<Tile>>;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Empty,
    Elf,
    MovingElves { prev: Vec<Pos> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

fn spread_rounds(map: &mut Map, rounds: usize) -> usize {
    let mut count = 0;
    for _ in map.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let mut map = parse(sample!());
    //part 1
    let res = spread_rounds(&mut map, 10);
    assert_eq!(res, 110);
    //part 2
    // let res = operation2(&map);
    // assert_eq!(res, 42);
}
