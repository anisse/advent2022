fn main() {
    let (crates, instructions) = parse(include_str!("../input.txt"));
    //part 1
    let res = movecrates(&mut crates, instructions);
    println!("Summary: {}", res);
    //part 2
    //let res = movecrates2(&(crates, instructions));
    //println!("Summary2: {}", res);
}

struct Move {
    qty: u16,
    src: u16,
    dst: u16,
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    input.lines().map(|x| x.parse().expect("not int")).collect()
}
fn movecrates(crates: &mut [Vec<char>], instructions: Vec<Move>) -> String {
    "".to_string()
}

#[test]
fn test() {
    let (crates, instructions) = parse(include_str!("../sample.txt"));
    //part 1
    let res = movecrates(&mut crates, instructions);
    assert_eq!(res, "CMZ");
    //part 2
    // let res = movecrates2(&(crates, instructions));
    // assert_eq!(res, 42);
}
