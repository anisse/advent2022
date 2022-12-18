use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Surface {
    dim: u8,
    x: u8,
    y: u8,
    z: u8,
}
impl Surface {
    fn new(dim: u8, x: u8, y: u8, z: u8) -> Self {
        Surface { dim, x, y, z }
    }
    fn sc(mut self, which: u8, inc: i16) -> Self {
        match (self.dim + which) % 3 {
            0 => self.x = (self.x as i16 + inc) as u8,
            1 => self.y = (self.y as i16 + inc) as u8,
            2 => self.z = (self.z as i16 + inc) as u8,
            _ => unreachable!(),
        }
        self
    }
    fn sd(mut self, inc: u8) -> Self {
        self.dim = (self.dim + inc) % 3;
        self
    }
    fn adjacent(&self) -> AdjacentSurfaceIterator {
        AdjacentSurfaceIterator {
            count: 0,
            s: self.clone(),
        }
    }
}
struct AdjacentSurfaceIterator {
    count: u8,
    s: Surface,
}
impl AdjacentSurfaceIterator {
    fn next_golden_x(&mut self) -> Option<Surface> {
        self.count += 1;
        if self.count == 12 {
            return None;
        }
        Some(match self.s.dim {
            0 => match self.count - 1 {
                // x dimension, x should NOT vary for same-plan surfaces
                0 => Surface {
                    y: self.s.y + 1,
                    ..self.s
                },
                1 => Surface {
                    y: self.s.y - 1,
                    ..self.s
                },
                2 => Surface {
                    z: self.s.z - 1,
                    ..self.s
                },
                3 => Surface {
                    z: self.s.z + 1,
                    ..self.s
                },
                // Plan y surfaces
                4 => Surface { dim: 1, ..self.s },
                5 => Surface {
                    dim: 1,
                    y: self.s.y + 1,
                    ..self.s
                },
                6 => Surface {
                    dim: 1,
                    x: self.s.x - 1,
                    ..self.s
                },
                7 => Surface {
                    dim: 1,
                    x: self.s.x - 1,
                    y: self.s.y + 1,
                    ..self.s
                },
                // Plan z surfaces
                8 => Surface { dim: 2, ..self.s },
                9 => Surface {
                    dim: 2,
                    z: self.s.z + 1,
                    ..self.s
                },
                10 => Surface {
                    dim: 2,
                    x: self.s.x - 1,
                    ..self.s
                },
                11 => Surface {
                    dim: 2,
                    x: self.s.x - 1,
                    z: self.s.z + 1,
                    ..self.s
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        })
    }
}
impl Iterator for AdjacentSurfaceIterator {
    type Item = Surface;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count == 12 {
            return None;
        }
        Some(match self.count - 1 {
            // x dimension, x should NOT vary for same-plan surfaces
            0 => self.s.clone().sc(1, 1),
            1 => self.s.clone().sc(1, -1),
            2 => self.s.clone().sc(2, 1),
            3 => self.s.clone().sc(2, -1),
            // Plan y surfaces
            4 => self.s.clone().sd(1),
            5 => self.s.clone().sc(1, 1).sd(1),
            6 => self.s.clone().sc(0, -1).sd(1),
            7 => self.s.clone().sc(0, -1).sc(1, 1).sd(1),
            // Plan z surfaces
            8 => self.s.clone().sd(2),
            9 => self.s.clone().sc(2, 1).sd(2),
            10 => self.s.clone().sc(0, -1).sd(2),
            11 => self.s.clone().sc(2, 1).sc(0, -1).sd(2),
            _ => unreachable!(),
        })
    }
}

type SurfaceIndex = HashMap<Surface, u8>;

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
        *surfaces.entry(Surface::new(0, x, y, z)).or_insert(0) += 1;
        *surfaces.entry(Surface::new(0, x + 1, y, z)).or_insert(0) += 1;
        *surfaces.entry(Surface::new(1, x, y, z)).or_insert(0) += 1;
        *surfaces.entry(Surface::new(1, x, y + 1, z)).or_insert(0) += 1;
        *surfaces.entry(Surface::new(2, x, y, z)).or_insert(0) += 1;
        *surfaces.entry(Surface::new(2, x, y, z + 1)).or_insert(0) += 1;
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
    let start = surfaces
        .iter()
        .filter(|(Surface { dim, .. }, v)| *dim == 0 && **v == 1)
        .fold(Surface::new(0, u8::MAX, 0, 0), |acc, (s, v)| {
            if s.x < acc.x && *v == 1 {
                s.clone()
            } else {
                acc
            }
        });
    let mut count = 0;
    let mut seen: HashSet<Surface> = HashSet::new();
    let mut next: Vec<Surface> = Vec::new();
    next.push(start);
    while let Some(s) = next.pop() {
        if seen.get(&s).is_some() {
            continue;
        }
        seen.insert(s.clone());
        for adj in s.adjacent() {
            println!("At adjacent surface of {s:?}: {adj:?}");
            if surfaces.get(&adj) == Some(&1) {
                println!("Unseen before surface adjascent of {s:?} : {adj:?}");
                next.push(adj);
            }
        }
        count += 1
    }
    count
}

#[test]
fn test() {
    let cubes = parse(sample!());
    //part 1
    let res = unexposed_surface(&cubes);
    assert_eq!(res, 64);
    //part 2
    let res = unexposed_exterior_surface(&cubes);
    assert_eq!(res, 58);
}
