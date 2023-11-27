use advent2022::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = operation(things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = operation2(things);
    println!("Part 2: {}", res);
}
type ParsedItem = u8;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| x.parse().expect("not int"))
}
fn operation<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut count = 0;
    for _ in things {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

fn operation2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    for _ in things {
        todo!()
    }
    0
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = operation(things.clone());
    assert_eq!(res, 42);
    //part 2
    let res = operation2(things);
    assert_eq!(res, 42);
}
