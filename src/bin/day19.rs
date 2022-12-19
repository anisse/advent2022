#[cfg(test)]
use std::time::Instant;

use advent2022::*;

#[macro_use]
extern crate scan_fmt;

use crate::Resource::*;

fn main() {
    let blueprints = parse(input!());
    //part 1
    let res = quality_levels(&blueprints, 24);
    println!("Summary: {}", res);
    //part 2
    let res = max_geodes_product(&blueprints[0..3], 32);
    println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().map(|b|  {
        let (_, ore_ore, clay_ore, obs_ore, obs_clay, geo_ore, geo_obs) = scan_fmt!(
            b,
            "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            String,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8
        ).expect("parse error");
        [
            Robot{
                cost: vec![Unit{
                    res: Ore,
                    n: ore_ore
                }],
            },
            Robot{
                cost: vec![Unit{
                    res: Ore,
                    n: clay_ore}],
            },
            Robot{
                cost: vec![
                    Unit{
                    res: Ore,
                    n: obs_ore,
                    },
                    Unit{
                    res: Clay,
                    n: obs_clay,
                    },
                ],
            },
            Robot{
                cost: vec![
                    Unit{
                    res: Ore,
                    n: geo_ore,
                    },
                    Unit{
                    res: Obsidian,
                    n: geo_obs,
                    },
                ],
            },

        ]
    }
        ).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Resource {
    Ore = 0,
    Clay,
    Obsidian,
    Geode,
}
impl From<usize> for Resource {
    fn from(value: usize) -> Self {
        match value {
            0 => Ore,
            1 => Clay,
            2 => Obsidian,
            3 => Geode,
            _ => panic!("impossible resource {value}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Unit {
    res: Resource,
    n: u8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Robot {
    cost: Vec<Unit>,
}

type Blueprint = [Robot; 4];

#[derive(Debug, PartialEq, Eq, Clone, Hash, Ord, PartialOrd, Default)]
struct State {
    robots: [u8; 4],
    resources: [u8; 4],
    budget: u8,
}
impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Minute {}: we have {:?} robots and {:?} resources",
            25 - self.budget,
            self.robots,
            self.resources
        )
    }
}

fn max_geodes_product(blueprints: &[Blueprint], l: u8) -> usize {
    blueprints
        .iter()
        .map(|b| {
            max_geodes(
                b,
                State {
                    robots: [1, 0, 0, 0],
                    resources: [0, 0, 0, 0],
                    budget: l,
                },
            )
        })
        .product()
}
fn quality_levels(blueprints: &[Blueprint], l: u8) -> usize {
    blueprints
        .iter()
        .map(|b| {
            max_geodes(
                b,
                State {
                    robots: [1, 0, 0, 0],
                    resources: [0, 0, 0, 0],
                    budget: l,
                },
            )
        })
        .enumerate()
        .map(|(i, m)| (i + 1) * m)
        .sum()
}

fn max_geodes(b: &Blueprint, s: State) -> usize {
    //println!("{s} for {b:?}");
    let default = s.resources[Geode as usize] as usize
        + s.robots[Geode as usize] as usize * (s.budget as usize);
    if s.budget == 0 {
        return default;
    }
    (0..4)
        .rev()
        .map(|i| {
            let (empty, enough) = b
                .iter()
                .flat_map(|r| r.cost.iter().filter(|c| c.res as usize == i).map(|c| c.n))
                .fold((true, true), |acc, n| (false, acc.1 && s.robots[i] >= n));
            if !empty && enough {
                // No need to produce any more of this: we already have more per turn than any
                // robots need
                return default;
            }
            // can we produce robot r ?
            // we have one of each robot of its resources
            if b[i].cost.iter().any(|c| s.robots[c.res as usize] == 0) {
                // Otherwise no point in continuing
                return default;
            }
            // With no other action, what is the time to produce this robot ?
            let cost = b[i]
                .cost
                .iter()
                .map(|c| {
                    if s.resources[c.res as usize] >= c.n {
                        0
                    } else {
                        let a = (c.n - s.resources[c.res as usize]) as u16;
                        let b = (s.robots[c.res as usize]) as u16;
                        //ceil div
                        ((a + b - 1) / b) as u8
                    }
                })
                .max()
                .expect("max")
                + 1;
            if cost >= s.budget {
                return default;
            }
            let mut new_robots = s.robots;
            new_robots[i] += 1;
            let mut new_s = State {
                budget: s.budget - cost,
                robots: new_robots,
                resources: s.resources,
            };
            // Update next resources
            (0..cost).for_each(|_| {
                s.robots
                    .iter()
                    .enumerate()
                    .for_each(|(r, ro)| new_s.resources[r] += ro)
            });
            b[i].cost
                .iter()
                .for_each(|c| new_s.resources[c.res as usize] -= c.n);
            /*
            space_indent(s.budget, 24);
            println!(
                "{s} producing {:?} for {cost} -> {new_s}",
                Resource::from(i)
            );
            */
            max_geodes(b, new_s)
        })
        .max()
        .expect("max")
}

#[test]
fn test() {
    let blueprints = parse(sample!());
    //part 1
    let start = Instant::now();
    assert_eq!(quality_levels(&blueprints[0..1], 24), 9, "BP 1");
    println!("P1 BP1 done in {:?}", start.elapsed());
    let start = Instant::now();
    assert_eq!(quality_levels(&blueprints[1..], 24), 12, "BP 2");
    println!("P1 BP2 done in {:?}", start.elapsed());
    //assert_eq!(quality_levels(&blueprints, 24), 33, "both BP");
    let input_blue = parse(input!());
    let start = Instant::now();
    assert_eq!(quality_levels(&input_blue, 24), 2301, "input BP");
    println!("input done in {:?}", start.elapsed());
    //part 2
    let start = Instant::now();
    assert_eq!(quality_levels(&blueprints[0..1], 32), 56, "Part 2 BP 1");
    println!("P2 BP 1 done in {:?}", start.elapsed());
    let start = Instant::now();
    assert_eq!(quality_levels(&blueprints[1..], 32), 62, "Part 2 BP 2");
    println!("P2 BP 2 done in {:?}", start.elapsed());
    //assert_eq!(max_geodes_product(&blueprints, 32), 62 * 56, "P2 both BP");
}
