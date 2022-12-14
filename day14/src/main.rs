fn main() {
    let rocklines = parse(include_str!("../input.txt"));
    //part 1
    let res = operation(&rocklines);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&rocklines);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<u8> {
    input.lines().map(|x| x.parse().expect("not int")).collect()
}
fn operation(rocklines: &[u8]) -> usize {
    let mut count = 0;
    for _ in rocklines.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let rocklines = parse(include_str!("../sample.txt"));
    //part 1
    let res = operation(&rocklines);
    assert_eq!(res, 24);
    //part 2
    // let res = operation2(&rocklines);
    // assert_eq!(res, 42);
}
