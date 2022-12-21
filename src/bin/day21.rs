use std::collections::HashMap;

use advent2022::*;

use crate::Op::*;

fn main() {
    let ops = parse(input!());
    //part 1
    let res = operation(&ops);
    println!("Summary: {}", res);
    //part 2
    let res = number_to_yell(&ops);
    println!("Summary2: {}", res);
}
fn parse(input: &str) -> CalcBook {
    input
        .lines()
        .map(|l| {
            let mut li = l.split(": ");
            let name = li.next().expect("a monkey name").to_string();
            let op = li.next().expect("op");
            (
                name,
                match op.chars().next().expect("an op char") {
                    '0'..='9' => Number(op.parse().expect("not int")),
                    _ => {
                        let opi: Vec<&str> = op.split(' ').collect();
                        match opi[1] {
                            "+" => Add(opi[0].to_string(), opi[2].to_string()),
                            "-" => Sub(opi[0].to_string(), opi[2].to_string()),
                            "/" => Div(opi[0].to_string(), opi[2].to_string()),
                            "*" => Mul(opi[0].to_string(), opi[2].to_string()),
                            _ => panic!("Unknown op {}", opi[1]),
                        }
                    }
                },
            )
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Number(i64),
    Div(String, String),
    Add(String, String),
    Mul(String, String),
    Sub(String, String),
}

impl Op {
    fn eval(&self, m: &CalcBook) -> i64 {
        fn ev(num: &str, m: &CalcBook) -> i64 {
            m.get(num).expect("a number").eval(m)
        }
        match self {
            Number(i) => *i,
            Div(a, b) => ev(a, m) / ev(b, m),
            Add(a, b) => ev(a, m) + ev(b, m),
            Mul(a, b) => ev(a, m) * ev(b, m),
            Sub(a, b) => ev(a, m) - ev(b, m),
        }
    }
    fn operands(&self) -> Option<(&str, &str)> {
        match self {
            Number(_) => None,
            Div(r1, r2) | Add(r1, r2) | Mul(r1, r2) | Sub(r1, r2) => Some((r1, r2)),
        }
    }
    fn has_op(&self, op: &str, ops: &CalcBook) -> bool {
        match self.operands() {
            Some((r1, r2)) => {
                let o1 = ops.get(r1).expect("o1");
                let o2 = ops.get(r2).expect("o2");
                r1 == op || r2 == op || o1.has_op(op, ops) || o2.has_op(op, ops)
            }
            None => false,
        }
    }
}

fn operation(ops: &CalcBook) -> i64 {
    ops.get("root").expect("root").eval(ops)
}

type CalcBook = HashMap<String, Op>;

fn number_to_yell(ops: &CalcBook) -> i64 {
    let root = ops.get("root").expect("root");
    let (r1, r2) = root.operands().expect("ops");
    let o1 = ops.get(r1).expect("o1");
    let o2 = ops.get(r2).expect("o2");
    println!(
        "Root has {r1} and {r2}. Humn is on r1 ? {} on r2 {} ?",
        o1.has_op("humn", ops),
        o2.has_op("humn", ops)
    );
    0
}

#[test]
fn test() {
    let ops = parse(sample!());
    //part 1
    let res = operation(&ops);
    assert_eq!(res, 152);
    //part 2
    let res = number_to_yell(&ops);
    assert_eq!(res, 301);
}
