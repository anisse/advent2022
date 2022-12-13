fn main() {
    let pairs = parse(include_str!("../input.txt"));
    //part 1
    let res = count_right_order(&pairs);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&pairs);
    //println!("Summary2: {}", res);
}

type Packet = Vec<u8>;
type Pair = [Packet; 2];

fn parse(input: &str) -> Vec<Pair> {
    input
        .split("\n\n")
        .map(|pair| {
            pair.lines()
                .map(|l| l.chars().map(|c| c as u8).collect::<Packet>())
                .collect::<Vec<Packet>>()
                .try_into()
                .expect("not two packets")
        })
        .collect()
}
fn count_right_order(pairs: &[Pair]) -> usize {
    let mut count = 0;
    for _ in pairs.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let pairs = parse(include_str!("../sample.txt"));
    //part 1
    let res = count_right_order(&pairs);
    assert_eq!(res, 42);
    //part 2
    // let res = operation2(&pairs);
    // assert_eq!(res, 42);
}
