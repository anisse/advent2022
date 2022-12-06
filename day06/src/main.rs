fn main() {
    let datastream = parse(include_str!("../input.txt"));
    //part 1
    let res = startofpacket(&datastream);
    println!("Start of packet: {}", res);
    //part 2
    let res = startofmessage(&datastream, 14);
    println!("Start of message: {}", res);
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
        match buf.len() {
            4 => {
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
            }
            0..=3 => buf.push(c),
            _ => unreachable!(),
        }
    }
    panic!("Not found");
}

fn startofmessage(datastream: &[char], len: usize) -> usize {
    'outer: for (i, _) in datastream.iter().enumerate().skip(len - 1) {
        for j in 0..len {
            for k in (j + 1)..len {
                if datastream[i - j] == datastream[i - k] {
                    continue 'outer;
                }
            }
        }
        return i + 1;
    }
    panic!("Not found");
}

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
    let test = parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
    assert_eq!(startofmessage(&test, 14), 19);
    let test = parse("bvwbjplbgvbhsrlpgdmjqwftvncz");
    assert_eq!(startofmessage(&test, 14), 23);
    let test = parse("nppdvjthqldpwncqszvftbrmjlhg");
    assert_eq!(startofmessage(&test, 14), 23);
    let test = parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
    assert_eq!(startofmessage(&test, 14), 29);
    let test = parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
    assert_eq!(startofmessage(&test, 14), 26);
}
