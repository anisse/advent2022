use std::{cmp::max, collections::HashMap};

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
    //let res = operation2(&blueprints);
    //println!("Summary2: {}", res);
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
                t: Ore,
                cost: vec![Unit{
                    res: Ore,
                    n: ore_ore
                }],
            },
            Robot{
                t: Clay,
                cost: vec![Unit{
                    res: Ore,
                    n: clay_ore}],
            },
            Robot{
                t: Obsidian,
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
                t: Geode,
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
    t: Resource,
    cost: Vec<Unit>,
}

type Blueprint = [Robot; 4];

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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

fn quality_levels(blueprints: &[Blueprint], l: u8) -> usize {
    blueprints
        .iter()
        .map(|b| {
            let mut max = vec![0; l as usize + 1];
            quality_level(
                b,
                State {
                    robots: [1, 0, 0, 0],
                    resources: [0, 0, 0, 0],
                    budget: l,
                },
                &mut max,
            )
        })
        .sum()
}

fn ore_equivalent(b: &Blueprint, s: &State) -> usize {
    let mut ore_equiv = [0_usize, 0, 0, 0];
    for (i, r) in b.iter().enumerate() {
        let mut cost = 0;
        for c in r.cost.iter() {
            if c.res == Ore {
                cost += c.n as usize;
            } else {
                cost += ore_equiv[c.res as usize];
            }
        }
        ore_equiv[i] = cost;
    }
    s.robots
        .iter()
        .enumerate()
        .map(|(i, ro)| ore_equiv[i] * s.budget as usize * *ro as usize)
        .sum::<usize>()
        + s.resources
            .iter()
            .enumerate()
            .map(|(i, res)| {
                if Resource::from(i) == Ore {
                    *res as usize
                } else {
                    ore_equiv[i] * *res as usize
                }
            })
            .sum::<usize>()
}
fn quality_level(b: &Blueprint, s: State, max_ore_equivalent: &mut Vec<usize>) -> usize {
    //println!("{s} for {b:?}");
    if s.budget == 0 {
        return s.resources[Geode as usize] as usize + s.robots[Geode as usize] as usize;
    }
    let oe = ore_equivalent(b, &s);
    space_indent(s.budget, 24);
    println!(
        "{s} OE is {oe}, max {}",
        max_ore_equivalent[s.budget as usize]
    );
    if max_ore_equivalent[s.budget as usize] > oe {
        return s.resources[Geode as usize] as usize
            + s.robots[Geode as usize] as usize * s.budget as usize;
    }
    max_ore_equivalent[s.budget as usize] = oe;
    (0..4)
        .rev()
        .map(|i| {
            // can we produce robot r ?
            // we have one of each robot of its resources
            if b[i].cost.iter().any(|c| s.robots[c.res as usize] == 0) {
                // Otherwise no point in continuing
                return 0;
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
                return 0;
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
            space_indent(s.budget, 24);
            println!(
                "{s} producing {:?} for {cost} -> {new_s}",
                Resource::from(i)
            );
            quality_level(b, new_s, max_ore_equivalent)
        })
        .max()
        .expect("max")
}

#[test]
fn test() {
    let blueprints = parse(sample!());
    //part 1
    let res = quality_levels(&blueprints, 24);
    assert_eq!(res, 33);
    //part 2
    // let res = operation2(&blueprints);
    // assert_eq!(res, 42);
}
