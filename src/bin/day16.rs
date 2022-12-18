use advent2022::*;
#[macro_use]
extern crate scan_fmt;

use std::cmp::max;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let valves = parse(input!());
    //part 1
    let res = most_30m_pressure(&valves);
    println!("Max flow in 30m: {}", res);
    //part 2
    let res = max_flow_with_elephant_fast(&valves);
    println!("Max flow in 26m with elephant: {}", res);
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy)]
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
            //println!("{}", l);
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

fn most_30m_pressure(valves: &HashMap<ValveName, Valve>) -> usize {
    let valves_with_flow: HashMap<ValveName, usize> = valves
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(name, v)| (*name, v.flow))
        .collect();

    /*
    valves_with_flow.iter().for_each(|(name, flow)| {
        let path = path_to_valve(valves, "AA".into(), *name);
        println!("From AA to reach {name} (flow:{flow}), path has len {path}",);
    });
    */
    let mut remain: VecDeque<ValveName> = valves_with_flow.keys().cloned().collect();
    let mut path_memo = HashMap::new();

    max_flow(valves, "AA".into(), &mut remain, 30, &mut path_memo)
}

fn max_flow(
    valves: &HashMap<ValveName, Valve>,
    at: ValveName,
    remaining: &mut VecDeque<ValveName>,
    budget: u8,
    path_memo: &mut HashMap<(ValveName, ValveName), u8>,
) -> usize {
    let mut max = 0;
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Movement {
    dest: ValveName,
    flow: usize,
    cost: u8,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Pos {
    at: ValveName,
    moving: Option<Movement>,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.moving {
            Some(m) => write!(f, "in transit to {} ({} to get {})", m.dest, m.cost, m.flow),
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
        let cost = path_to_valve(valves, at, to) + 1; // add cost of turning directly here
        path_memo.insert((at, to), cost);
        cost
    }
}

#[allow(dead_code)]
fn space_indent(budget: u8) {
    (0..(26 - budget)).for_each(|_| print! {" "});
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
        mincost.insert(valve, cost);
        if valve == target {
            return cost;
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

type BitFlow = HashMap<u64, usize>;
type ValveMap = HashMap<ValveName, Valve>;
type PathMemo = HashMap<(ValveName, ValveName), u8>;

fn max_flow_with_elephant_fast(valves: &ValveMap) -> usize {
    let mut path_memo = HashMap::new();
    let valves_with_flow: Vec<ValveName> = valves
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(name, _)| *name)
        .collect();
    let valve_bits: HashMap<ValveName, u64> = valves_with_flow
        .iter()
        .scan(0, |acc, valve| {
            *acc += 1;
            Some((*valve, 1 << (*acc - 1)))
        })
        .collect();
    /*
    valve_bits.iter().for_each(|(v, b)| {
        println!("{v} has bit {b:08X}");
    });
    */
    let mut bitflow = BitFlow::new();
    flow_all(
        valves,
        &valve_bits,
        &mut path_memo,
        &valves_with_flow,
        &mut bitflow,
        "AA".into(),
        26,
        0,
        0,
    );
    bitflow
        .iter()
        .flat_map(|(s1, f1)| {
            bitflow
                .iter()
                .filter(|(s2, _)| (*s1 & *s2) == 0)
                .map(|(_, f2)| *f1 + *f2)
        })
        .max()
        .expect("max")
}
#[allow(clippy::too_many_arguments)]
fn flow_all(
    valves: &ValveMap,
    valve_bits: &HashMap<ValveName, u64>,
    path_memo: &mut PathMemo,
    remain: &[ValveName],
    bitflow: &mut BitFlow,
    at: ValveName,
    budget: u8,
    state: u64,
    flow: usize,
) {
    let f = bitflow.get(&state).unwrap_or(&0);
    bitflow.insert(state, max(*f, flow));

    for dest in remain.iter().cloned() {
        /*
        space_indent(budget);
        println!(
            "Evaluating {at}({}) -> {dest}({}) (state: {state:08X}",
            at.bit(),
            dest.bit(),
        );
        */
        let db = valve_bits.get(&dest).expect("a bitmask");
        if db & state != 0 {
            continue;
        }
        let v = valves.get(&dest).expect("dest");
        let cost = cost(at, dest, valves, path_memo);
        if cost >= budget {
            continue;
        }
        let new_budget = budget - cost;
        flow_all(
            valves,
            valve_bits,
            path_memo,
            remain,
            bitflow,
            dest,
            new_budget,
            state | db,
            flow + (new_budget as usize * v.flow),
        );
    }
}

#[test]
fn test() {
    let valves = parse(sample!());
    //part 1
    let res = most_30m_pressure(&valves);
    assert_eq!(res, 1651);
    //part 2
    let res = max_flow_with_elephant_fast(&valves);
    assert_eq!(res, 1707);
}

#[test]
fn community_inputs() {
    for (input, p1, p2) in [
        (
            "Valve AA has flow rate=0; tunnels lead to valves BA
Valve BA has flow rate=2; tunnels lead to valves AA, CA
Valve CA has flow rate=4; tunnels lead to valves BA, DA
Valve DA has flow rate=6; tunnels lead to valves CA, EA
Valve EA has flow rate=8; tunnels lead to valves DA, FA
Valve FA has flow rate=10; tunnels lead to valves EA, GA
Valve GA has flow rate=12; tunnels lead to valves FA, HA
Valve HA has flow rate=14; tunnels lead to valves GA, IA
Valve IA has flow rate=16; tunnels lead to valves HA, JA
Valve JA has flow rate=18; tunnels lead to valves IA, KA
Valve KA has flow rate=20; tunnels lead to valves JA, LA
Valve LA has flow rate=22; tunnels lead to valves KA, MA
Valve MA has flow rate=24; tunnels lead to valves LA, NA
Valve NA has flow rate=26; tunnels lead to valves MA, OA
Valve OA has flow rate=28; tunnels lead to valves NA, PA
Valve PA has flow rate=30; tunnels lead to valves OA",
            2640,
            2670,
        ),
        (
            "Valve AA has flow rate=0; tunnels lead to valves BA
Valve BA has flow rate=1; tunnels lead to valves AA, CA
Valve CA has flow rate=4; tunnels lead to valves BA, DA
Valve DA has flow rate=9; tunnels lead to valves CA, EA
Valve EA has flow rate=16; tunnels lead to valves DA, FA
Valve FA has flow rate=25; tunnels lead to valves EA, GA
Valve GA has flow rate=36; tunnels lead to valves FA, HA
Valve HA has flow rate=49; tunnels lead to valves GA, IA
Valve IA has flow rate=64; tunnels lead to valves HA, JA
Valve JA has flow rate=81; tunnels lead to valves IA, KA
Valve KA has flow rate=100; tunnels lead to valves JA, LA
Valve LA has flow rate=121; tunnels lead to valves KA, MA
Valve MA has flow rate=144; tunnels lead to valves LA, NA
Valve NA has flow rate=169; tunnels lead to valves MA, OA
Valve OA has flow rate=196; tunnels lead to valves NA, PA
Valve PA has flow rate=225; tunnels lead to valves OA",
            13468,
            12887,
        ),
        (
            "Valve BA has flow rate=2; tunnels lead to valves AA, CA
Valve CA has flow rate=10; tunnels lead to valves BA, DA
Valve DA has flow rate=2; tunnels lead to valves CA, EA
Valve EA has flow rate=10; tunnels lead to valves DA, FA
Valve FA has flow rate=2; tunnels lead to valves EA, GA
Valve GA has flow rate=10; tunnels lead to valves FA, HA
Valve HA has flow rate=2; tunnels lead to valves GA, IA
Valve IA has flow rate=10; tunnels lead to valves HA, JA
Valve JA has flow rate=2; tunnels lead to valves IA, KA
Valve KA has flow rate=10; tunnels lead to valves JA, LA
Valve LA has flow rate=2; tunnels lead to valves KA, MA
Valve MA has flow rate=10; tunnels lead to valves LA, NA
Valve NA has flow rate=2; tunnels lead to valves MA, OA
Valve OA has flow rate=10; tunnels lead to valves NA, PA
Valve PA has flow rate=2; tunnels lead to valves OA, AA
Valve AA has flow rate=0; tunnels lead to valves BA, PA",
            1288,
            1484,
        ),
        (
            "Valve AA has flow rate=0; tunnels lead to valves AB, BB, CB
Valve AB has flow rate=0; tunnels lead to valves AA, AC
Valve AC has flow rate=0; tunnels lead to valves AB, AD
Valve AD has flow rate=0; tunnels lead to valves AC, AE
Valve AE has flow rate=0; tunnels lead to valves AD, AF
Valve AF has flow rate=0; tunnels lead to valves AE, AG
Valve AG has flow rate=0; tunnels lead to valves AF, AH
Valve AH has flow rate=0; tunnels lead to valves AG, AI
Valve AI has flow rate=0; tunnels lead to valves AH, AJ
Valve AJ has flow rate=0; tunnels lead to valves AI, AK
Valve AK has flow rate=100; tunnels lead to valves AJ, AW, AX, AY, AZ
Valve AW has flow rate=10; tunnels lead to valves AK
Valve AX has flow rate=10; tunnels lead to valves AK
Valve AY has flow rate=10; tunnels lead to valves AK
Valve AZ has flow rate=10; tunnels lead to valves AK
Valve BB has flow rate=0; tunnels lead to valves AA, BC
Valve BC has flow rate=0; tunnels lead to valves BB, BD
Valve BD has flow rate=0; tunnels lead to valves BC, BE
Valve BE has flow rate=0; tunnels lead to valves BD, BF
Valve BF has flow rate=0; tunnels lead to valves BE, BG
Valve BG has flow rate=0; tunnels lead to valves BF, BH
Valve BH has flow rate=0; tunnels lead to valves BG, BI
Valve BI has flow rate=0; tunnels lead to valves BH, BJ
Valve BJ has flow rate=0; tunnels lead to valves BI, BK
Valve BK has flow rate=100; tunnels lead to valves BJ, BW, BX, BY, BZ
Valve BW has flow rate=10; tunnels lead to valves BK
Valve BX has flow rate=10; tunnels lead to valves BK
Valve BY has flow rate=10; tunnels lead to valves BK
Valve BZ has flow rate=10; tunnels lead to valves BK
Valve CB has flow rate=0; tunnels lead to valves AA, CC
Valve CC has flow rate=0; tunnels lead to valves CB, CD
Valve CD has flow rate=0; tunnels lead to valves CC, CE
Valve CE has flow rate=0; tunnels lead to valves CD, CF
Valve CF has flow rate=0; tunnels lead to valves CE, CG
Valve CG has flow rate=0; tunnels lead to valves CF, CH
Valve CH has flow rate=0; tunnels lead to valves CG, CI
Valve CI has flow rate=0; tunnels lead to valves CH, CJ
Valve CJ has flow rate=0; tunnels lead to valves CI, CK
Valve CK has flow rate=100; tunnels lead to valves CJ, CW, CX, CY, CZ
Valve CW has flow rate=10; tunnels lead to valves CK
Valve CX has flow rate=10; tunnels lead to valves CK
Valve CY has flow rate=10; tunnels lead to valves CK
Valve CZ has flow rate=10; tunnels lead to valves CK",
            2400,
            3680,
        ),
    ]
    .iter()
    {
        let valves = parse(input);
        assert_eq!(most_30m_pressure(&valves), *p1, "Part 1 failed");
        assert_eq!(max_flow_with_elephant_fast(&valves), *p2, "Part 2 failed");
        println!("=========================================== OK !!!!!!!!");
    }
}
