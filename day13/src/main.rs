use crate::DataType::*;
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::{cmp::Ord, ops::Range};

fn main() {
    let pairs = parse(include_str!("../input.txt"));
    //part 1
    let res = count_right_order(&pairs);
    println!("Packets pair in the right order: {}", res);
    //part 2
    let res = order_all(&pairs);
    println!("Product of indices of market packets after sort: {}", res);
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Packet(Vec<u8>);
type Pair = [Packet; 2];

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|c| *c as char).collect::<String>()
        )
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_slice(other, 0..self.0.len(), 0..other.0.len())
    }
}

impl Packet {
    fn cmp_slice(&self, other: &Self, rs: Range<usize>, ro: Range<usize>) -> Ordering {
        let left = first(&self.0[rs.start..]);
        let right = first(&other.0[ro.start..]);
        //println!("Comparing {left:?} ({rs:?}) with {right:?} ({ro:?})");
        match (left.typ, right.typ) {
            (List, List) => self.cmp_lists(
                other,
                (rs.start + 1)..(rs.start + left.len),
                (ro.start + 1)..(ro.start + right.len),
            ),
            (List, Int(_)) => self.cmp_lists(
                other,
                (rs.start + 1)..(rs.start + left.len),
                ro.start..(ro.start + right.len),
            ),
            (Int(_), List) => self.cmp_lists(
                other,
                rs.start..(rs.start + left.len),
                (ro.start + 1)..(ro.start + right.len),
            ),
            (Int(value), Int(valueo)) => value.cmp(&valueo),
        }
    }
    fn cmp_lists(&self, other: &Self, rs: Range<usize>, ro: Range<usize>) -> Ordering {
        let mut s_start = rs.start;
        let mut o_start = ro.start;
        loop {
            //println!("left is {s_start} of {rs:?}, right {o_start} of {ro:?}");
            /* Compare both first just in case one is an empty list */
            let pair = (s_start.cmp(&rs.end), o_start.cmp(&ro.end));
            /*
            println!(
                "Ends are : {pair:?}: {} vs {} and {} vs {}",
                s_start, rs.end, o_start, ro.end
            );
            */
            match pair {
                // continue
                (Less, Less) => {
                    // nothing to do
                }
                (Less, Equal) => return Greater,
                (Less, Greater) => return Greater,
                (Equal, Less) => return Less,
                (Equal, Equal) => return Equal,
                (Equal, Greater) => {
                    panic!("right end greater than ro.end")
                }
                (Greater, Less) => return Less,
                (Greater, Equal) => {
                    panic!("left end greater than rs.end")
                }
                (Greater, Greater) => return Equal,
            }
            let left = first(&self.0[s_start..rs.end]);
            let right = first(&other.0[o_start..ro.end]);
            let cmp = self.cmp_slice(other, s_start..left.len, o_start..right.len);
            match cmp {
                Less => return cmp,
                Greater => return cmp,
                Equal => {}
            }
            s_start += left.len
                + match left.typ {
                    List => 2,   // list ended, add '],' length
                    Int(_) => 1, // int ended, add ',' length
                };
            o_start += right.len
                + match right.typ {
                    List => 2,   // list ended, add '],' length
                    Int(_) => 1, // int ended, add ',' length
                };
        }
        //Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
enum DataType {
    List,
    Int(usize),
}
#[derive(Debug)]
struct Data {
    len: usize,
    typ: DataType,
}

fn first(p: &[u8]) -> Data {
    let int = p.iter().take_while(|c| (b'0'..=b'9').contains(c)).count();
    if int > 0 {
        assert!(int == p.len() || p[int] == b',' || p[int] == b']');
        return Data {
            typ: Int(p[..int]
                .iter()
                .map(|c| *c as char)
                .collect::<String>()
                .parse()
                .expect("not int")),
            len: int,
        };
    }
    //println!("not int next is {}", p[0] as char);
    assert_eq!(p[0], b'[');
    let len = p[1..]
        .iter()
        .scan(1, |state, c| {
            match c {
                b'[' => *state += 1,
                b']' => *state -= 1,
                _ => {}
            }
            Some(*state)
        })
        .take_while(|count| *count > 0)
        .count()
        + 1;
    Data { len, typ: List }
}

fn parse(input: &str) -> Vec<Pair> {
    input
        .split("\n\n")
        .map(|pair| {
            pair.lines()
                .map(|l| Packet(l.chars().map(|c| c as u8).collect::<Vec<u8>>()))
                .collect::<Vec<Packet>>()
                .try_into()
                .expect("not two packets")
        })
        .collect()
}
fn count_right_order(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .enumerate()
        /*
        .filter(|(i, pair)| {
            println!(
                "{i}: {} vs {}: {:?}",
                pair[0],
                pair[1],
                pair[0].cmp(&pair[1])
            );
            true
        })
        */
        .filter(|(_, pair)| pair[0] < pair[1])
        .map(|(i, _)| i + 1)
        .sum()
}

fn order_all(pairs: &[Pair]) -> usize {
    let m1 = Packet("[[2]]".chars().map(|c| c as u8).collect());
    let m2 = Packet("[[6]]".chars().map(|c| c as u8).collect());
    let mut packets: Vec<Packet> = vec![[m1.clone(), m2.clone()]]
        .iter()
        .chain(pairs.iter())
        .flatten()
        .cloned()
        .collect();
    packets.sort();
    packets
        .iter()
        .enumerate()
        .filter(|(_, p)| **p == m1 || **p == m2)
        .map(|(i, _)| i + 1)
        .product()
}

#[test]
fn test() {
    let pairs = parse(include_str!("../sample.txt"));
    //part 1
    let res = count_right_order(&pairs);
    assert_eq!(res, 13);
    //part 2
    let res = order_all(&pairs);
    assert_eq!(res, 140);
}
