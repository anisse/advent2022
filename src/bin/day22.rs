use advent2022::*;

//use itertools::Itertools;

use crate::Move::*;
use crate::Tile::*;

fn main() {
    let (map, pass) = parse(input!());
    //part 1
    let res = operation(&map, &pass);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&map, &pass);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> (Map, Password) {
    let mut inp = input.split("\n\n");
    let mapi = inp.next().expect("map");
    let passi = inp
        .next()
        .expect("password")
        .lines()
        .next()
        .expect("password line")
        .chars();
    let map: Map = mapi
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    ' ' => OffMap,
                    '.' => Open,
                    '#' => Wall,
                    _ => panic!("unknown char {c}"),
                })
                .collect()
        })
        .collect();
    let mut pass: Password = vec![];
    let mut ns = String::new();
    for c in passi {
        if c.is_ascii_digit() {
            ns.push(c);
            continue;
        }
        let n: u8 = ns.parse().expect("an int");
        ns.clear();
        pass.push(Forward(n));
        pass.push(match c {
            'L' => Left,
            'R' => Right,
            _ => panic!("unknown dir {c}"),
        })
    }
    let n: u8 = ns.parse().expect("an int");
    ns.clear();
    pass.push(Forward(n));

    (map, pass)
}

type Map = Vec<Vec<Tile>>;
type MapSlice = [Vec<Tile>];

type Password = Vec<Move>;
type PasswordSlice = [Move];

#[derive(Debug)]
enum Move {
    Forward(u8),
    Left,
    Right,
}

#[derive(Debug)]
enum Tile {
    OffMap,
    Open,
    Wall,
}

fn operation(m: &MapSlice, p: &PasswordSlice) -> usize {
    let mut count = 0;
    for _ in m.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let (map, pass) = parse(sample!());
    //part 1
    dbg!(&map);
    dbg!(&pass);
    let res = operation(&map, &pass);
    assert_eq!(res, 42);
    //part 2
    // let res = operation2(&map, &pass);
    // assert_eq!(res, 42);
}
