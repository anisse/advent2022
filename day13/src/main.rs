use std::{cmp::Ord, ops::Range};

fn main() {
    let pairs = parse(include_str!("../input.txt"));
    //part 1
    let res = count_right_order(&pairs);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&pairs);
    //println!("Summary2: {}", res);
}

#[derive(PartialEq, Eq, Debug)]
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
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp_slice(other, 0..self.0.len(), 0..other.0.len())
    }
}

impl Packet {
    fn cmp_slice(&self, other: &Self, rs: Range<usize>, ro: Range<usize>) -> std::cmp::Ordering {
        let left = first(&self.0[rs.start..]);
        let right = first(&other.0[ro.start..]);
        println!("Comparing {left:?} ({rs:?}) with {right:?} ({ro:?})");
        match (left, right) {
            (Data::List { end }, Data::List { end: endo }) => {
                self.cmp_slice(other, (rs.start + 1)..(end), (ro.start + 1)..endo)
            }
            (Data::List { end }, Data::Int { value, .. }) => {
                let new_p = Packet(format!("{value}").chars().map(|c| c as u8).collect());
                self.cmp_slice(&new_p, rs.start..end, 0..new_p.0.len())
            }
            (Data::Int { value, .. }, Data::List { end: endo }) => {
                let new_p = Packet(format!("{value}").chars().map(|c| c as u8).collect());
                new_p.cmp_slice(other, 0..new_p.0.len(), rs.start..endo)
            }
            (
                Data::Int { value, end },
                Data::Int {
                    value: valueo,
                    end: endo,
                },
            ) => {
                let c = value.cmp(&valueo);
                match c {
                    std::cmp::Ordering::Less => c,
                    std::cmp::Ordering::Greater => c,
                    std::cmp::Ordering::Equal => match (end.cmp(&rs.end), endo.cmp(&ro.end)) {
                        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => self.cmp_slice(
                            other,
                            (rs.start + end + 1)..rs.end,
                            (ro.start + endo + 1)..ro.end,
                        ),
                        (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                            std::cmp::Ordering::Less
                        }
                        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
                            panic!("endo greater than ro.end")
                        }
                        (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => {
                            std::cmp::Ordering::Greater
                        }
                        (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => {
                            std::cmp::Ordering::Equal
                        }
                        (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => {
                            panic!("endo greater than ro.end")
                        }
                        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
                            panic!("end greater than rs.end")
                        }
                        (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => {
                            panic!("end greater than rs.end")
                        }
                        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                            panic!("endo and end greater")
                        }
                    },
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
enum Data {
    List { end: usize },
    Int { value: u16, end: usize },
}

fn first(p: &[u8]) -> Data {
    let int = p.iter().take_while(|c| (b'0'..=b'9').contains(c)).count();
    if int > 0 {
        assert!(int == p.len() || p[int] == b',' || p[int] == b']');
        return Data::Int {
            value: p[..int]
                .iter()
                .map(|c| *c as char)
                .collect::<String>()
                .parse()
                .expect("not int"),
            end: int,
        };
    }
    assert_eq!(p[0], b'[');
    let end = p[1..]
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
        .count();
    /*
        .last()
        .expect("no list end");
    */
    Data::List { end }
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
        .filter(|pair| {
            println!("{} vs {}", pair[0], pair[1]);
            true
        })
        .filter(|pair| pair[0] < pair[1])
        .count()
}

#[test]
fn test() {
    let pairs = parse(include_str!("../sample.txt"));
    //part 1
    let res = count_right_order(&pairs);
    assert_eq!(res, 13);
    //part 2
    // let res = operation2(&pairs);
    // assert_eq!(res, 42);
}
