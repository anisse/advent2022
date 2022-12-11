use std::fmt::Display;

fn main() {
    let monkeys = parse(include_str!("../input.txt"));
    //part 1
    let res = monkey_business(&monkeys);
    println!("Monkey business: {}", res);
    //part 2
    let res = monkey_business_2(&monkeys);
    println!("Monkey biz 2: {}", res);
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    div: u64,
    pass_true: usize,
    pass_false: usize,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "items: ")?;
        for i in self.items.iter() {
            write!(f, "{i} ")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Op {
    Mul(u64),
    Add(u64),
    Square,
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|m| {
            let mut l = m.lines().skip(1);
            //l.next().expect("no monkey");
            let items = l
                .next()
                .expect("no items")
                .split(": ")
                .nth(1)
                .expect("no item list")
                .split(", ")
                .map(|it| it.parse().expect("not int"))
                .collect();
            let mut lop = l
                .next()
                .expect("no op line")
                .split(" = ")
                .nth(1)
                .expect("no op")
                .split_ascii_whitespace();
            assert_eq!(lop.next().expect("no old"), "old", "old should be old");
            let op = match lop.next().expect("+ or *") {
                "+" => Op::Add(
                    lop.next()
                        .expect("no add argument")
                        .parse()
                        .expect("add not int"),
                ),
                "*" => {
                    let arg = lop.next().expect("no mul argument");
                    match arg {
                        "old" => Op::Square,
                        _ => Op::Mul(arg.parse().expect("mul arg not int")),
                    }
                }
                _ => panic!("Unexpected operation"),
            };
            let div = l
                .next()
                .expect("no div line")
                .split_ascii_whitespace()
                .nth(3)
                .expect("no div")
                .parse()
                .expect("div not int");
            let pass_true = l
                .next()
                .expect("no pass_true line")
                .split_ascii_whitespace()
                .nth(5)
                .expect("no pass_true")
                .parse()
                .expect("pass_true not int");
            let pass_false = l
                .next()
                .expect("no pass_false line")
                .split_ascii_whitespace()
                .nth(5)
                .expect("no pass_false")
                .parse()
                .expect("pass_false not int");
            Monkey {
                items,
                op,
                div,
                pass_true,
                pass_false,
            }
        })
        .collect()
}
fn monkey_business(monkeys: &[Monkey]) -> usize {
    monkey_business_common(monkeys, 20, &|x| x / 3)
}

fn common_divisible(monkeys: &[Monkey]) -> u64 {
    monkeys.iter().map(|m| m.div).product()
}
fn monkey_business_2(monkeys: &[Monkey]) -> usize {
    let common = common_divisible(monkeys);
    monkey_business_common(monkeys, 10000, &|x| x % common)
}

fn monkey_business_common(monkeys: &[Monkey], rounds: usize, op: &dyn Fn(u64) -> u64) -> usize {
    let mut monkeys = monkeys.to_vec();
    let mut counts: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            counts[i] += monkeys[i].items.len();
            monkey_turn_common(&mut monkeys, i, op);
        }
    }

    counts.sort_by(|a, b| b.cmp(a));
    counts.iter().take(2).product()
}

fn monkey_turn_common(monkeys: &mut [Monkey], m: usize, control: &dyn Fn(u64) -> u64) {
    let (op, div) = (monkeys[m].op.clone(), monkeys[m].div);
    let (pass_true, pass_false) = (monkeys[m].pass_true, monkeys[m].pass_false);
    #[allow(clippy::needless_collect)] // borrow checker workaround
    let send: Vec<(usize, u64)> = monkeys[m]
        .items
        .drain(0..)
        .map(|item| {
            let level = control(match op {
                Op::Mul(x) => item * x,
                Op::Add(x) => item + x,
                Op::Square => item * item,
            });
            let dest = if level % div == 0 {
                pass_true
            } else {
                pass_false
            };
            (dest, level)
        })
        .collect();
    send.into_iter()
        .for_each(|(dest, item)| monkeys[dest].items.push(item));
}

#[test]
fn test() {
    let monkeys = parse(include_str!("../sample.txt"));
    //part 1
    let res = monkey_business(&monkeys);
    assert_eq!(res, 10605);
    //part 2
    let res = monkey_business_2(&monkeys);
    assert_eq!(res, 2713310158);
}
