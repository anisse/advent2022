fn main() {
    let datastream = parse(include_str!("../input.txt"));
    //part 1
    let res = startofpacket(&datastream);
    println!("Start of packet: {}", res);
    //part 2
    //let res = operation2(&datastream);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<char> {
    input
        .lines()
        .next()
        .expect("no first line")
        .chars()
        .collect()
}
fn startofpacket(datastream: &[char]) -> usize {
    let mut count = 0;
    for _ in datastream.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let datastream = parse(include_str!("../sample.txt"));
    //part 1
    let res = startofpacket(&datastream);
    assert_eq!(res, 7);
    //part 2
    // let res = operation2(&datastream);
    // assert_eq!(res, 42);
}
