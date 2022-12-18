use std::collections::HashMap;

use advent2022::*;
fn main() {
    let cubes = parse(input!());
    //part 1
    let res = unexposed_surface(&cubes);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&cubes);
    //println!("Summary2: {}", res);
}

type Cube = Vec<u8>;

fn parse(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().expect("not int")).collect())
        .collect()
}
fn unexposed_surface(cubes: &[Cube]) -> usize {
    let mut surfaces_xy: HashMap<(u8, u8, u8), u8> = HashMap::new();
    let mut surfaces_xz: HashMap<(u8, u8, u8), u8> = HashMap::new();
    let mut surfaces_yz: HashMap<(u8, u8, u8), u8> = HashMap::new();
    for c in cubes.iter() {
        let x = c[0];
        let y = c[1];
        let z = c[2];
        *surfaces_xy.entry((x, y, z)).or_insert(0) += 1;
        *surfaces_xy.entry((x, y, z + 1)).or_insert(0) += 1;
        *surfaces_xz.entry((x, y, z)).or_insert(0) += 1;
        *surfaces_xz.entry((x, y + 1, z)).or_insert(0) += 1;
        *surfaces_yz.entry((x, y, z)).or_insert(0) += 1;
        *surfaces_yz.entry((x + 1, y, z)).or_insert(0) += 1;
    }
    surfaces_xy
        .values()
        .chain(surfaces_xz.values())
        .chain(surfaces_yz.values())
        .filter(|v| **v == 1)
        .count()
}

#[test]
fn test() {
    let cubes = parse(sample!());
    //part 1
    let res = unexposed_surface(&cubes);
    assert_eq!(res, 64);
    //part 2
    // let res = operation2(&cubes);
    // assert_eq!(res, 42);
}
