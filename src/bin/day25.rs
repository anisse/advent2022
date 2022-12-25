use advent2022::*;
fn main() {
    let snafus = parse(input!());
    //part 1
    let res = sum(&snafus);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&snafus);
    //println!("Summary2: {}", res);
}
type Snafu = Vec<char>;
fn parse(input: &str) -> Vec<Snafu> {
    input.lines().map(|x| x.parse().expect("not int")).collect()
}
fn sum(snafus: &[Snafu]) -> usize {
    let mut count = 0;
    for _ in snafus.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let snafus = parse(sample!());
    //part 1
    let res = sum(&snafus);
    assert_eq!(res, 4890);
    //part 2
    // let res = operation2(&snafus);
    // assert_eq!(res, 42);
}
