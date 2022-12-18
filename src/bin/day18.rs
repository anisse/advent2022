use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use advent2022::*;
fn main() {
    let cubes = parse(input!());
    //part 1
    let res = unexposed_surface(&cubes);
    println!("Unconnected voxel cube surfaces: {}", res);
    //part 2
    let res = unexposed_exterior_surface(&cubes);
    println!("Lava cube external area: {}", res);
}

type Cube = Vec<u8>;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Surface {
    dim: u8,
    x: u8,
    y: u8,
    z: u8,
}
impl std::fmt::Display for Surface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({:2},{:2},{:2})",
            match self.dim {
                0 => "X",
                1 => "Y",
                2 => "Z",
                _ => unreachable!(),
            },
            self.x,
            self.y,
            self.z,
        )
    }
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
    fn adj_edges_iter(&self, o: bool) -> EdgeAdjIterator {
        EdgeAdjIterator {
            count: 0,
            s: self.clone(),
            o,
        }
    }
}
struct EdgeAdjIterator {
    count: u8,
    s: Surface,
    o: bool,
}

impl Iterator for EdgeAdjIterator {
    type Item = EdgeAdj;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count == 4 {
            return None;
        }
        Some(EdgeAdj {
            s: self.s.clone(),
            n: self.count - 1,
            o: self.o,
        })
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct EdgeAdj {
    s: Surface,
    n: u8,
    o: bool,
}
impl EdgeAdj {
    fn surfaces_adj(&self) -> EdgeSurfaceIterator {
        EdgeSurfaceIterator {
            e: self.clone(),
            count: 0,
        }
    }
}
struct EdgeSurfaceIterator {
    count: u8,
    e: EdgeAdj,
}
impl Iterator for EdgeSurfaceIterator {
    type Item = Surface;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count == 4 {
            return None;
        }
        let x = if self.e.o {
            self.count - 1
        } else {
            3 - self.count
        };
        Some(match self.e.n {
            0 => match x {
                0 => self.e.s.clone().sc(1, 1).sd(1),
                1 => self.e.s.clone().sc(1, 1),
                2 => self.e.s.clone().sc(1, 1).sc(0, -1).sd(1),
                _ => unreachable!(),
            },
            1 => match x {
                0 => self.e.s.clone().sc(2, 1).sd(2),
                1 => self.e.s.clone().sc(2, 1),
                2 => self.e.s.clone().sc(2, 1).sc(0, -1).sd(2),
                _ => unreachable!(),
            },
            2 => match x {
                0 => self.e.s.clone().sd(1),
                1 => self.e.s.clone().sc(1, -1),
                2 => self.e.s.clone().sc(0, -1).sd(1),
                _ => unreachable!(),
            },
            3 => match x {
                0 => self.e.s.clone().sd(2),
                1 => self.e.s.clone().sc(2, -1),
                2 => self.e.s.clone().sc(0, -1).sd(2),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        })
    }
}

type SurfaceIndex = HashMap<Surface, Vec<bool>>;

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
        surfaces
            .entry(Surface::new(0, x, y, z))
            .or_default()
            .push(false);
        surfaces
            .entry(Surface::new(0, x + 1, y, z))
            .or_default()
            .push(true);
        surfaces
            .entry(Surface::new(1, x, y, z))
            .or_default()
            .push(false);
        surfaces
            .entry(Surface::new(1, x, y + 1, z))
            .or_default()
            .push(true);
        surfaces
            .entry(Surface::new(2, x, y, z))
            .or_default()
            .push(false);
        surfaces
            .entry(Surface::new(2, x, y, z + 1))
            .or_default()
            .push(true);
    }
    surfaces
}
fn unexposed_surface(cubes: &[Cube]) -> usize {
    let surfaces = unexposed_surface_common(cubes);
    surfaces.values().filter(|v| v.len() == 1).count()
}
fn print_surfaces_advance(seen: &HashSet<Surface>, current: &Surface, dim: u8) {
    if seen.len() <= 1 {
        return;
    }
    let (min_x, max_x) = seen.iter().map(|s| s.x).minmax().into_option().unwrap();
    let (min_y, max_y) = seen.iter().map(|s| s.y).minmax().into_option().unwrap();

    let surfaces: HashSet<(u8, u8)> = seen
        .iter()
        .filter(|s| s.dim == dim)
        .map(|s| (s.x, s.y))
        .collect();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if surfaces.get(&(x, y)).is_some() {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
fn unexposed_exterior_surface(cubes: &[Cube]) -> usize {
    let surfaces = unexposed_surface_common(cubes);
    // Lets find a min surface and start iterating from there
    let (start, orient) = surfaces
        .iter()
        .filter(|(Surface { dim, .. }, v)| *dim == 0 && v.len() == 1)
        .min_by(|a, b| {
            a.0.x
                .cmp(&b.0.x)
                .then(a.0.y.cmp(&b.0.y))
                .then(a.0.z.cmp(&b.0.z))
        })
        .expect("min");
    /*
    .fold((Surface::new(0, u8::MAX, 0, 0), true), |acc, (s, v)| {
        if s.x < acc.0.x && v.len() == 1 {
            (s.clone(), v[0])
        } else {
            acc
        }
    });
    */
    assert_eq!(orient.len(), 1);
    assert!(!orient[0]);
    let mut count = 0;
    let mut seen: HashSet<Surface> = HashSet::new();
    let mut next: Vec<(Surface, bool)> = Vec::new();
    next.push((start.clone(), orient[0]));
    while let Some((s, o)) = next.pop() {
        if seen.get(&s).is_some() {
            continue;
        }
        //println!("Exploring... {s} with o={o}  ");
        seen.insert(s.clone());
        for (i, e) in s.adj_edges_iter(o).enumerate() {
            //println!("edge {i}");
            for adj in e.surfaces_adj() {
                //println!("Surface adjascent of {s} : {adj}");
                if let Some(os) = surfaces.get(&adj) {
                    //println!("Surface adjascent of {s} : {adj} {}", os[0]);
                    if os.len() == 1 {
                        //print_surfaces_advance(&seen, &adj, 0);
                        next.push((adj, os[0]));
                        break;
                    }
                }
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
