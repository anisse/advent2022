use std::collections::HashMap;

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
            "{} minutes remaining: we have {:?} robots and {:?} resources",
            self.budget, self.robots, self.resources
        )
    }
}

fn quality_levels(blueprints: &[Blueprint], l: u8) -> usize {
    blueprints
        .iter()
        .map(|b| {
            let mut memo = HashMap::new();
            quality_level(
                b,
                State {
                    robots: [1, 0, 0, 0],
                    resources: [0, 0, 0, 0],
                    budget: l,
                },
                &mut memo,
            )
        })
        .sum()
}

fn quality_level(b: &Blueprint, mut s: State, seen: &mut HashMap<State, usize>) -> usize {
    //println!("{s} for {b:?}");
    if s.budget == 0 {
        return s.resources[Geode as usize] as usize;
    }
    // Update resources
    s.robots
        .iter()
        .enumerate()
        .for_each(|(i, ro)| s.resources[i] += ro);
    //println!("{s}");
    (0..4)
        .rev()
        .map(|i| {
            let mut new_robots = s.robots;
            // can we produce robot r ?
            if b[i].cost.iter().all(|c| c.n < s.resources[c.res as usize]) {
                new_robots[i] += 1;
            }
            let new_s = State {
                budget: s.budget - 1,
                robots: new_robots,
                resources: s.resources,
            };
            if let Some(quality) = seen.get(&new_s) {
                *quality
            } else {
                let q = quality_level(b, new_s.clone(), seen);
                seen.insert(new_s, q);
                q
            }
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
