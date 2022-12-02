fn main() {
    let strategy_guide = parse(include_str!("../input.txt"));
    //part 1
    let score = compute_score(&strategy_guide);
    println!("Summary: {}", score);
    //part 2
    //let score = operation2(&strategy_guide);
    //println!("Summary2: {}", score);
}
fn parse(input: &str) -> Vec<u8> {
    input.lines().map(|x| x.parse().expect("not int")).collect()
}
fn compute_score(strategy_guide: &[u8]) -> usize {
    let mut count = 0;
    for _ in strategy_guide.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let strategy_guide = parse(include_str!("../sample.txt"));
    //part 1
    let score = compute_score(&strategy_guide);
    assert_eq!(score, 15);
    //part 2
    // let score = operation2(&strategy_guide);
    // assert_eq!(score, 42);
}
