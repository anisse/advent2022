use std::collections::HashMap;

use advent2022::*;

use crate::Op::*;

fn main() {
    let ops = parse(input!());
    //part 1
    let res = operation(&ops);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&ops);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> HashMap<String, Op> {
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
    fn eval(&self, m: &HashMap<String, Op>) -> i64 {
        match self {
            Number(i) => *i,
            Div(a, b) => m.get(a).expect("a").eval(m) / m.get(b).expect("b").eval(m),
            Add(a, b) => m.get(a).expect("a").eval(m) + m.get(b).expect("b").eval(m),
            Mul(a, b) => m.get(a).expect("a").eval(m) * m.get(b).expect("b").eval(m),
            Sub(a, b) => m.get(a).expect("a").eval(m) - m.get(b).expect("b").eval(m),
        }
    }
}

fn operation(ops: &HashMap<String, Op>) -> i64 {
    ops.get("root").expect("root").eval(ops)
}

#[test]
fn test() {
    let ops = parse(sample!());
    //part 1
    let res = operation(&ops);
    assert_eq!(res, 152);
    //part 2
    // let res = operation2(&ops);
    // assert_eq!(res, 42);
}
