use advent2022::*;
#[macro_use]
extern crate scan_fmt;

use std::collections::BinaryHeap;
use std::collections::HashMap;

//use crate::Action::*;

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
type ValveName = String;

#[derive(Debug, Clone)]
struct Valve {
    flow: usize,
    tunnels: Vec<ValveName>,
}

fn parse(input: &str) -> HashMap<String, Valve> {
    input
        .lines()
        .map(|l| {
            println!("{}", l);
            let (name, flow, _, _, _, tunnels) = scan_fmt!(
                l,
                "Valve {} has flow rate={d}; {} {} to {} {/.*/}{e}",
                String,
                usize,
                String,
                String,
                String,
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

/*
#[derive(Debug, Clone)]
enum Action {
    Open(ValveName),
    GoTo(ValveName),
}
*/
#[derive(Debug, Clone)]
struct State {
    pos: ValveName,
    //path: Vec<Action>,
    state: HashMap<ValveName, bool>,
    flow: usize,
}

fn most_30m_pressure(valves: &HashMap<ValveName, Valve>) -> usize {
    let mut state = State {
        pos: "AA".to_string(),
        //path: Vec::new(),
        state: HashMap::new(),
        flow: 0,
    };
    let valves_with_flow: HashMap<ValveName, usize> = valves
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(name, v)| (name.clone(), v.flow))
        .collect();

    valves_with_flow.iter().for_each(|(name, flow)| {
        let path = path_to_valve(valves, "AA".to_string(), name.clone());
        println!("From AA to reach {name} (flow:{flow}), path has len {path}",);
    });

    0
}

fn path_to_valve(valves: &HashMap<ValveName, Valve>, start: ValveName, target: ValveName) -> usize {
    // Here we need shortest path, let's start with that
    #[derive(Eq, PartialEq)]
    struct VNext {
        valve: ValveName,
        cost: u8,
    }
    impl Ord for VNext {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }
    impl PartialOrd for VNext {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    //let mut costpath: HashMap<u8, ValveName> = HashMap::new();
    let mut mincost: HashMap<ValveName, u8> = HashMap::new();
    let mut next: BinaryHeap<VNext> = BinaryHeap::new();

    next.push(VNext {
        valve: start,
        cost: 0,
    });
    while let Some(VNext { valve, cost }) = next.pop() {
        if let Some(min) = mincost.get(&valve) {
            if cost >= *min {
                continue;
            }
        }
        //costpath.insert(cost, valve.clone());
        mincost.insert(valve.clone(), cost);
        if valve == target {
            return cost as usize;
            /*
            return (0..=cost)
                .map(|c| costpath.get(&c).expect("some path"))
                .map(|v| GoTo(v.to_string()))
                .collect();
            */
        }
        for n in valves.get(&valve).expect("some valve").tunnels.iter() {
            next.push(VNext {
                valve: n.to_string(),
                cost: cost + 1,
            });
        }
    }
    usize::MAX
}

struct OpenValve {
    name: ValveName,
    flow: usize,
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
