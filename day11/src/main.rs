fn main() {
    let monkeys = parse(include_str!("../input.txt"));
    //part 1
    let res = monkey_business(&monkeys);
    println!("Monkey business: {}", res);
    //part 2
    //let res = operation2(&monkeys);
    //println!("Summary2: {}", res);
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u8>,
    op: Op,
    div: usize,
    pass_true: u8,
    pass_false: u8,
}

#[derive(Debug)]
enum Op {
    Mul(usize),
    Add(usize),
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

    //.map(|x| x.parse().expect("not int"))
}
fn monkey_business(monkeys: &[Monkey]) -> usize {
    let mut count = 0;
    for _ in monkeys.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let monkeys = parse(include_str!("../sample.txt"));
    //part 1
    dbg!(&monkeys);
    let res = monkey_business(&monkeys);
    assert_eq!(res, 10605);
    //part 2
    // let res = operation2(&monkeys);
    // assert_eq!(res, 42);
}
