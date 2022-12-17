use crate::Jet::*;
use crate::Tile::*;
use advent2022::*;

fn main() {
    let jets = parse(input!());
    //part 1
    let res = simulate(&jets, 2022);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&jets);
    //println!("Summary2: {}", res);
}
#[derive(Debug, Clone)]
enum Tile {
    Stone,
    Space,
}
type Rock = Vec<Vec<Tile>>;

fn p(s: &str) -> Vec<Tile> {
    s.chars()
        .map(|c| match c {
            '#' => Stone,
            _ => Space,
        })
        .collect()
}
fn rocks() -> Vec<Rock> {
    vec![
        vec![p("####")],
        vec![p(".#."), p("###"), p(".#.")],
        vec![p("..#"), p("..#"), p("###")],
        vec![p("#"), p("#"), p("#"), p("#")],
        vec![p("##"), p("##")],
    ]
}

enum Jet {
    Left,
    Right,
}

fn parse(input: &str) -> Vec<Jet> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Right,
            '<' => Left,
            _ => unreachable!(),
        })
        .collect()
}
fn simulate(jets: &[Jet], rounds: usize) -> usize {
    let rocks = rocks();
    0
}

#[test]
fn test() {
    let jets = parse(sample!());
    //part 1
    let res = simulate(&jets, 2022);
    assert_eq!(res, 3068);
    //part 2
    // let res = operation2(&jets);
    // assert_eq!(res, 42);
}
