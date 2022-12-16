use advent2022::*;
#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;

fn main() {
    let valves = parse(input!());
    //part 1
    let res = most_30m_pressure(&valves);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&valves);
    //println!("Summary2: {}", res);
}

/*
struct ValveName(u16);
impl From<String> for ValveName
*/

struct Valve {
    flow: usize,
    tunnels: Vec<String>,
}

fn parse(input: &str) -> HashMap<String, Valve> {
    input
        .lines()
        .map(|l| {
            let (name, flow, tunnels) = scan_fmt!(
                l,
                "Valve {} has flow rate={}; tunnels lead to valves {}",
                String,
                usize,
                String
            )
            .expect("parse error");

            (
                name,
                Valve {
                    flow,
                    tunnels: tunnels.split(", ").map(|t| t.to_string()).collect(),
                },
            )
        })
        .collect()
}
fn most_30m_pressure(valves: &HashMap<String, Valve>) -> usize {
    let mut count = 0;
    for _ in valves.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let valves = parse(sample!());
    //part 1
    let res = most_30m_pressure(&valves);
    assert_eq!(res, 1651);
    //part 2
    // let res = operation2(&valves);
    // assert_eq!(res, 42);
}
