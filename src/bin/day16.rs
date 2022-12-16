use advent2022::*;
#[macro_use]
extern crate scan_fmt;

use std::cmp::min;
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

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct ValveName(u16);
impl std::fmt::Display for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            (self.0 & 0xFF) as u8 as char,
            ((self.0 >> 8) & 0xFF) as u8 as char
        )
    }
}
impl std::fmt::Debug for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<&str> for ValveName {
    fn from(s: &str) -> Self {
        Self(
            s.chars().next().expect("first") as u8 as u16
                | ((s.chars().nth(1).expect("second") as u8 as u16) << 8),
        )
    }
}

#[derive(Debug, Clone)]
struct Valve {
    flow: usize,
    tunnels: Vec<ValveName>,
}

fn parse(input: &str) -> HashMap<ValveName, Valve> {
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
                name[..].into(),
                Valve {
                    flow,
                    tunnels: tunnels.split(", ").map(|t| t.into()).collect(),
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
        .map(|(name, v)| (*name, v.flow))
        .collect();

    valves_with_flow.iter().for_each(|(name, flow)| {
        let path = path_to_valve(valves, "AA".into(), *name);
        println!("From AA to reach {name} (flow:{flow}), path has len {path}",);
    });
    let mut remain: VecDeque<ValveName> = valves_with_flow.iter().map(|(v, _)| *v).collect();
    let mut path_memo = HashMap::new();

    max_flow(valves, "AA".into(), &mut remain, 30, &mut path_memo)
}
fn max_flow_with_elephant(valves: &HashMap<ValveName, Valve>) -> usize {
    let valves_with_flow: HashMap<ValveName, usize> = valves
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(name, v)| (*name, v.flow))
        .collect();

    valves_with_flow.iter().for_each(|(name, flow)| {
        let path = path_to_valve(valves, "AA".into(), *name);
        println!("From AA to reach {name} (flow:{flow}), path has len {path}",);
    });
    let remain: Vec<ValveName> = valves_with_flow.iter().map(|(v, _)| *v).collect();
    let mut path_memo = HashMap::new();

    max_flow_double(
        valves,
        &[
            Pos {
                at: "AA".into(),
                moving: None,
            },
            Pos {
                at: "AA".into(),
                moving: None,
            },
        ],
        &remain,
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
        let cost = if let Some(cost) = path_memo.get(&(at, r)) {
            *cost
        } else {
            let cost = path_to_valve(valves, at, r);
            path_memo.insert((at, r), cost);
            cost
        };
        //let cost = path_to_valve(valves, at, r);
        if cost + 1 >= budget {
            remaining.push_back(r);
            continue;
        }
        let new_budget = budget - cost - 1; // cost of turning - 1
        let mflow = max_flow(valves, r, remaining, new_budget, path_memo);
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

fn cost(
    at: ValveName,
    to: ValveName,
    valves: &HashMap<ValveName, Valve>,
    path_memo: &mut HashMap<(ValveName, ValveName), u8>,
) -> u8 {
    if let Some(cost) = path_memo.get(&(at, to)) {
        *cost
    } else {
        let cost = path_to_valve(valves, at, to);
        path_memo.insert((at, to), cost);
        cost
    }
}

fn next_element(
    pos: &Pos,
    remaining: &[ValveName],
    idx: usize,
    valves: &HashMap<ValveName, Valve>,
    path_memo: &mut HashMap<(ValveName, ValveName), u8>,
) -> (ValveName, usize, u8) {
    if let Some(Movement { dest, flow, cost }) = &pos.moving {
        (*dest, *flow, *cost)
    } else {
        let dest = remaining[idx];
        let v = valves.get(&dest).expect("some valve");
        (dest, v.flow, cost(pos.at, dest, valves, path_memo))
    }
}
fn space_indent(budget: u8) {
    (0..(26 - budget)).for_each(|_| print! {" "});
}

fn move_pos(pos: &Pos, dest: ValveName, flow: usize, cost: u8, new_cost: u8) -> Pos {
    if new_cost == cost {
        // pos reached
        Pos {
            at: dest,
            moving: None,
        }
    } else {
        assert!(cost > new_cost);
        Pos {
            at: pos.at,
            moving: Some(Movement {
                dest,
                flow,
                cost: cost - new_cost,
            }),
        }
    }
}

fn sub_vec<T>(v: &[T], r: std::ops::RangeInclusive<usize>, removed: usize) -> Vec<T>
where
    T: Clone,
{
    assert!(r.contains(&removed));
    assert!(*r.end() < v.len());
    if *r.start() >= *r.end() {
        Vec::new()
    } else if removed == *r.start() {
        v[(removed + 1)..=*r.end()].to_vec()
    } else if removed == *r.end() {
        v[*r.start()..removed].to_vec()
    } else {
        let mut v2 = Vec::with_capacity(r.end() - r.start());
        v2.extend_from_slice(&v[*r.start()..removed]);
        v2.extend_from_slice(&v[(removed + 1)..=*r.end()]);
        v2
    }
}
fn sub_vec2<T>(v: &[T], rm1: Option<usize>, rm2: Option<usize>) -> Vec<T>
where
    T: Clone,
{
    let mut v2 = v.to_vec();
    if let Some(r2) = rm2 {
        v2.swap_remove(r2);
    }
    if let Some(r1) = rm1 {
        v2.swap_remove(r1);
    }
    v2
}

#[test]
fn test_sub_vec() {
    let v = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(sub_vec(&v, 1..=5, 1), vec![3, 4, 5, 6]);
    assert_eq!(sub_vec(&v, 0..=5, 1), vec![1, 3, 4, 5, 6]);
    assert_eq!(sub_vec(&v, 0..=5, 0), vec![2, 3, 4, 5, 6]);
    assert_eq!(sub_vec(&v, 0..=5, 2), vec![1, 2, 4, 5, 6]);
    assert_eq!(sub_vec(&v, 0..=3, 3), vec![1, 2, 3]);
    assert_eq!(sub_vec(&v, 0..=5, 3), vec![1, 2, 3, 5, 6]);
    assert_eq!(sub_vec(&v, 0..=4, 4), vec![1, 2, 3, 4,]);
}

fn max_flow_double(
    valves: &HashMap<ValveName, Valve>,
    pos: &[Pos; 2],
    remaining: &[ValveName],
    budget: u8,
    path_memo: &mut HashMap<(ValveName, ValveName), u8>,
) -> usize {
    let mut max = 0;
    space_indent(budget);
    println!(
        "budget is {budget} from {pos:?}: remain {} :{remaining:?} ",
        remaining.len()
    );
    let len = remaining.len();
    let mut i = 0;
    while i < len {
        space_indent(budget);
        println!("outer: {i} / {len}");
        let (r, vflow, cost) = next_element(&pos[0], remaining, i, valves, path_memo);
        let mut j = if pos[0].moving.is_none() { i + 1 } else { 0 };
        while j < len {
            let (r_ele, vflow_ele, cost_ele) =
                next_element(&pos[1], remaining, j, valves, path_memo);

            if cost + 1 >= budget && cost_ele + 1 >= budget {
                continue;
            }
            space_indent(budget);
            println!("inner: {j} / {len} - comparing {cost} with {cost_ele}");
            let (next_cost, next_vflow) = if cost <= cost_ele {
                (cost, vflow)
            } else {
                (cost_ele, vflow_ele)
            };
            let (new_budget, flow, new_pos) = (
                budget - next_cost - 1,
                next_vflow + if next_cost == cost_ele { vflow_ele } else { 0 },
                [
                    move_pos(&pos[0], r, vflow, cost, next_cost),
                    move_pos(&pos[1], r_ele, vflow_ele, cost_ele, next_cost),
                ],
            );

            let next_remain = match (&pos[0].moving, &pos[1].moving) {
                (None, None) => sub_vec2(remaining, Some(i), Some(j)),
                (None, Some(_)) => sub_vec2(remaining, Some(i), None),
                (Some(_), None) => sub_vec2(remaining, None, Some(j)),
                (Some(_), Some(_)) => unreachable!(),
            };
            space_indent(budget);
            println!("{{{i}, {j}}} / {len}: pos {pos:?}->{new_pos:?} rest: {next_remain:?}");
            let mflow = max_flow_double(valves, &new_pos, &next_remain, new_budget, path_memo);
            let new_flow = (new_budget as usize) * flow + mflow;
            space_indent(budget);
            println!(
                "= has flow {new_flow} = {new_budget} * {flow} + {mflow} at step {} with {pos:?}->{new_pos:?}",
                27 - budget
            );
            if new_flow > max {
                max = new_flow;
                space_indent(budget);
                println!("IS MAX ===");
            }
            if pos[1].moving.is_some() {
                break;
            } else {
                j += 1;
            }
        }
        if pos[0].moving.is_some() {
            break;
        } else {
            i += 1;
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
        //costpath.insert(cost, valve);
        mincost.insert(valve, cost);
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
                valve: *n,
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
