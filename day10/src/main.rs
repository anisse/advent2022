use crate::Ins::*;

fn main() {
    let instructions = parse(include_str!("../input.txt"));
    //part 1
    let res = signal_strength(&instructions);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&instructions);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<Ins> {
    input
        .lines()
        .map(|x| match &x[..4] {
            "noop" => Noop,
            "addx" => Addx(x[5..].parse().expect("not int")),
            _ => panic!("not instruction"),
        })
        .collect()
}

enum Ins {
    Noop,
    Addx(i8),
}
fn signal_strength(instructions: &[Ins]) -> usize {
    let mut count = 0;
    for _ in instructions.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let instructions = parse(include_str!("../sample.txt"));
    //part 1
    let res = signal_strength(&instructions);
    assert_eq!(res, 13140);
    //part 2
    // let res = operation2(&instructions);
    // assert_eq!(res, 42);
}
