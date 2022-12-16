use advent2022::*;
#[macro_use]
extern crate scan_fmt;

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;

//use crate::Action::*;

fn main() {
    let valves = parse(input!());
    //part 1
    let res = most_30m_pressure(&valves);
    println!("Summary: {}", res);
    //part 2
    let res = max_flow_with_elephant(&valves);
    println!("Summary2: {}", res);
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
#[derive(Debug, Clone)]
struct State {
    pos: ValveName,
    //path: Vec<Action>,
    state: HashMap<ValveName, bool>,
    flow: usize,
}
*/

fn most_30m_pressure(valves: &HashMap<ValveName, Valve>) -> usize {
    /*
    let mut state = State {
        pos: "AA".to_string(),
        //path: Vec::new(),
        state: HashMap::new(),
        flow: 0,
    };
    */
    let valves_with_flow: HashMap<ValveName, usize> = valves
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(name, v)| (name.clone(), v.flow))
        .collect();

    valves_with_flow.iter().for_each(|(name, flow)| {
        let path = path_to_valve(valves, "AA".to_string(), name.clone());
        println!("From AA to reach {name} (flow:{flow}), path has len {path}",);
    });
    let mut remain: VecDeque<ValveName> = valves_with_flow.iter().map(|(v, _)| v.clone()).collect();
    let mut path_memo = HashMap::new();

    max_flow(valves, "AA".to_string(), &mut remain, 30, &mut path_memo)
}
fn max_flow_with_elephant(valves: &HashMap<ValveName, Valve>) -> usize {
    let valves_with_flow: HashMap<ValveName, usize> = valves
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(name, v)| (name.clone(), v.flow))
        .collect();

    valves_with_flow.iter().for_each(|(name, flow)| {
        let path = path_to_valve(valves, "AA".to_string(), name.clone());
        println!("From AA to reach {name} (flow:{flow}), path has len {path}",);
    });
    let mut remain: VecDeque<ValveName> = valves_with_flow.iter().map(|(v, _)| v.clone()).collect();
    let mut path_memo = HashMap::new();

    max_flow_double(
        valves,
        [
            Pos {
                at: "AA".to_string(),
                moving: None,
            },
            Pos {
                at: "AA".to_string(),
                moving: None,
            },
        ],
        &mut remain,
        26,
        &mut path_memo,
    )
}

fn max_flow(
    valves: &HashMap<ValveName, Valve>,
    at: ValveName,
    remaining: &mut VecDeque<ValveName>,
    budget: u8,
    path_memo: &mut HashMap<(ValveName, ValveName), u8>,
) -> usize {
    let mut max = 0;
    //let mut max_through = ValveName::new();
    //print!(" budget is {budget} from {at}: remain {} ", remaining.len());
    for _ in 0..remaining.len() {
        let r = remaining.pop_front().expect("a valve name");
        let v = valves.get(&r).expect("some valve");

        /*
        (0..(30 - budget)).for_each(|_| print! {" "});
        println!("popped {r} of len {}", remaining.len());
        */
        let cost = if let Some(cost) = path_memo.get(&(at.clone(), r.clone())) {
            *cost
        } else {
            let cost = path_to_valve(valves, at.clone(), r.clone());
            path_memo.insert((at.clone(), r.clone()), cost);
            cost
        };
        //let cost = path_to_valve(valves, at.clone(), r.clone());
        if cost + 1 >= budget {
            remaining.push_back(r);
            continue;
        }
        let new_budget = budget - cost - 1; // cost of turning - 1
        let mflow = max_flow(valves, r.clone(), remaining, new_budget, path_memo);
        assert!(v.flow != 0);
        let flow = (new_budget as usize) * v.flow + mflow;
        /*
        (0..(30 - budget)).for_each(|_| print! {" "});
        println!("{i}: at {at}->{r} : ->{through} has flow {flow}");
        */
        if flow > max {
            max = flow;
            /*
            (0..(30 - budget)).for_each(|_| print! {" "});
            println!(" ==== IS MAX ===");
            */
        }
        /*
        (0..(30 - budget)).for_each(|_| print! {" "});
        println!("putting back {r} at the end");
        */
        remaining.push_back(r);
    }
    //println!("got flow of {max}");
    max
}

fn next(
    at: ValveName,
    valves: &HashMap<ValveName, Valve>,
    remaining: &mut VecDeque<ValveName>,
    path_memo: &mut HashMap<(ValveName, ValveName), u8>,
) -> (String, usize, u8) {
    let r = remaining.pop_front().expect("a valve name");
    let v = valves.get(&r).expect("some valve");

    let cost = if let Some(cost) = path_memo.get(&(at.clone(), r.clone())) {
        *cost
    } else {
        let cost = path_to_valve(valves, at.clone(), r.clone());
        path_memo.insert((at, r.clone()), cost);
        cost
    };
    (r.clone(), v.flow, cost)
}

#[derive(Debug)]
struct Movement {
    dest: ValveName,
    flow: usize,
    cost: u8,
}

//#[derive()]
struct Pos {
    at: ValveName,
    moving: Option<Movement>,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.moving {
            Some(m) => write!(f, "in transit to {} ({})", m.dest, m.cost),
            None => write!(f, "at {}", self.at),
        }
    }
}
impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn nexter(
    pos: &Pos,
    taken: &mut i8,
    valves: &HashMap<ValveName, Valve>,
    remaining: &mut VecDeque<ValveName>,
    path_memo: &mut HashMap<(ValveName, ValveName), u8>,
) -> (String, usize, u8, bool) {
    if let Some(Movement { dest, flow, cost }) = &pos.moving {
        (dest.clone(), *flow, *cost, false)
    } else if remaining.len() as i8 > *taken {
        *taken += 1;
        let (r, v, c) = next(pos.at.clone(), valves, remaining, path_memo);
        print!("taken {r} for {pos:?} (tot: {taken})");
        (r, v, c, true)
    } else {
        (pos.at.clone(), 0, 0, false)
    }
}
fn space_indent(budget: u8) {
    (0..(26 - budget)).for_each(|_| print! {" "});
}
fn max_flow_double(
    valves: &HashMap<ValveName, Valve>,
    pos: [Pos; 2],
    remaining: &mut VecDeque<ValveName>,
    budget: u8,
    path_memo: &mut HashMap<(ValveName, ValveName), u8>,
) -> usize {
    let mut max = 0;
    space_indent(budget);
    println!(
        "budget is {budget} from {pos:?}: remain {} :{remaining:?} ",
        remaining.len()
    );
    let mut taken = 0;
    'outer: while (taken as usize) < remaining.len() {
        space_indent(budget);
        let (r, vflow, cost, t) = nexter(&pos[0], &mut taken, valves, remaining, path_memo);
        println!(" - self, outer");
        let mut taken_inner = 0;
        while (taken_inner as usize) < remaining.len() {
            space_indent(budget);
            let (r_ele, vflow_ele, cost_ele, t_ele) =
                nexter(&pos[1], &mut taken_inner, valves, remaining, path_memo);
            println!(" - ele, inner");

            if cost + 1 >= budget && cost_ele + 1 >= budget {
                space_indent(budget);
                if t_ele {
                    remaining.push_back(r_ele.clone());
                    println!(
                        "skipping inner {r_ele}, putting at the end {taken_inner}/{}",
                        remaining.len()
                    );
                    continue;
                }
                println!();
                space_indent(budget);
                if t {
                    println!(
                        "skipping outer {r}, putting at the end {taken}/{}",
                        remaining.len()
                    );
                    remaining.push_back(r.clone());
                    continue 'outer;
                } else {
                    println!("skipping outer {r}, stopping");
                    break 'outer;
                }
            }
            let (new_budget, flow, new_pos) = match cost.cmp(&cost_ele) {
                std::cmp::Ordering::Less => (
                    budget - cost - 1, // cost of turning - 1
                    vflow,
                    [
                        Pos {
                            at: r.clone(),
                            moving: None,
                        },
                        Pos {
                            at: pos[1].at.clone(),
                            moving: Some(Movement {
                                dest: r_ele.clone(),
                                flow: vflow_ele,
                                cost: cost_ele - cost,
                            }),
                        },
                    ],
                ),
                std::cmp::Ordering::Equal => (
                    budget - cost - 1, // cost of turning - 1
                    vflow + vflow_ele,
                    [
                        Pos {
                            at: r.clone(),
                            moving: None,
                        },
                        Pos {
                            at: r_ele.clone(),
                            moving: None,
                        },
                    ],
                ),
                std::cmp::Ordering::Greater => (
                    budget - cost_ele - 1, // cost of turning - 1
                    vflow_ele,
                    [
                        Pos {
                            at: pos[0].at.clone(),
                            moving: Some(Movement {
                                dest: r.clone(),
                                flow: vflow,
                                cost: cost - cost_ele,
                            }),
                        },
                        Pos {
                            at: r_ele.clone(),
                            moving: None,
                        },
                    ],
                ),
            };
            space_indent(budget);
            println!("{taken_inner}/{}: at {pos:?}->{new_pos:?}", remaining.len());
            let mflow = max_flow_double(valves, new_pos, remaining, new_budget, path_memo);
            let new_flow = (new_budget as usize) * flow + mflow;
            space_indent(budget);
            println!("= has flow {new_flow}");
            if new_flow > max {
                max = new_flow;
                space_indent(budget);
                println!("IS MAX ===");
            }
            if t_ele {
                space_indent(budget);
                println!("done with inner {r_ele}, putting at the end");
                remaining.push_back(r_ele);
            } else {
                break;
            }
        }
        if t {
            space_indent(budget);
            println!("done with {r}, putting at the end");
            remaining.push_back(r);
        } else {
            break;
        }
    }
    space_indent(budget);
    println!("got flow of {max}");
    max
}

fn path_to_valve(valves: &HashMap<ValveName, Valve>, start: ValveName, target: ValveName) -> u8 {
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
            return cost;
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
    u8::MAX
}

#[test]
fn test() {
    let valves = parse(sample!());
    //part 1
    let res = most_30m_pressure(&valves);
    assert_eq!(res, 1651);
    //part 2
    let res = max_flow_with_elephant(&valves);
    assert_eq!(res, 1707);
}
