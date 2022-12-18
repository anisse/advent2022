use std::collections::HashMap;

use advent2022::*;
fn main() {
    let cubes = parse(input!());
    //part 1
    let res = unexposed_surface(&cubes);
    println!("Summary: {}", res);
    //part 2
    let res = unexposed_exterior_surface(&cubes);
    println!("Summary2: {}", res);
}

type Cube = Vec<u8>;
type SurfaceIndex = HashMap<(u8, u8, u8, u8), u8>;

fn parse(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().expect("not int")).collect())
        .collect()
}
fn unexposed_surface_common(cubes: &[Cube]) -> SurfaceIndex {
    let mut surfaces: SurfaceIndex = SurfaceIndex::new();
    for c in cubes.iter() {
        let x = c[0];
        let y = c[1];
        let z = c[2];
        *surfaces.entry((0, x, y, z)).or_insert(0) += 1;
        *surfaces.entry((0, x, y, z + 1)).or_insert(0) += 1;
        *surfaces.entry((1, x, y, z)).or_insert(0) += 1;
        *surfaces.entry((1, x, y + 1, z)).or_insert(0) += 1;
        *surfaces.entry((2, x, y, z)).or_insert(0) += 1;
        *surfaces.entry((2, x + 1, y, z)).or_insert(0) += 1;
    }
    surfaces
}
fn unexposed_surface(cubes: &[Cube]) -> usize {
    let surfaces = unexposed_surface_common(cubes);
    surfaces.values().filter(|v| **v == 1).count()
}
fn unexposed_exterior_surface(cubes: &[Cube]) -> usize {
    let surfaces = unexposed_surface_common(cubes);
    // Lets find a min surface and start iterating from there
    42
}

#[test]
fn test() {
    let cubes = parse(sample!());
    //part 1
    let res = unexposed_surface(&cubes);
    assert_eq!(res, 64);
    //part 2
    let res = unexposed_exterior_surface(&cubes);
    assert_eq!(res, 59);
}
