use std::collections::BinaryHeap;

fn main() {
    let map = parse(include_str!("../input.txt"));
    //part 1
    let res = shortest_path(&map);
    println!("Summary: {}", res);
    //part 2
    //let res = shortest_path2(&map);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|x| x.chars().map(|c| c as u8).collect())
        .collect()
}
fn shortest_path(map: &[Vec<u8>]) -> usize {
    let mut count = 0;
    for _ in map.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let map = parse(include_str!("../sample.txt"));
    //part 1
    let res = shortest_path(&map);
    assert_eq!(res, 42);
    //part 2
    // let res = shortest_path2(&map);
    // assert_eq!(res, 42);
}
