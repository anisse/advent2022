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
    let mut buf = Vec::new();
    for (i, c) in datastream.iter().enumerate() {
        if buf.len() == 4 {
            if buf[0] != buf[1]
                && buf[0] != buf[2]
                && buf[0] != buf[3]
                && buf[1] != buf[2]
                && buf[1] != buf[3]
                && buf[2] != buf[3]
            {
                return i;
            }
            buf.remove(0);
            buf.push(c);
        } else if buf.len() < 4 {
            buf.push(c)
        }
    }
    panic!("Not found");
    0
}

fn startofmessage(datastream: &[char], len: usize) -> usize {}

#[test]
fn test() {
    let datastream = parse(include_str!("../sample.txt"));
    //part 1
    let res = startofpacket(&datastream);
    assert_eq!(res, 7);
    let test2 = parse("bvwbjplbgvbhsrlpgdmjqwftvncz");
    assert_eq!(startofpacket(&test2), 5);
    let test3 = parse("nppdvjthqldpwncqszvftbrmjlhg");
    assert_eq!(startofpacket(&test3), 6);
    let test4 = parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
    assert_eq!(startofpacket(&test4), 10);
    //part 2
    // let res = operation2(&datastream);
    // assert_eq!(res, 42);
}
