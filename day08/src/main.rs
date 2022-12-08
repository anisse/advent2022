fn main() {
    let trees = parse(include_str!("../input.txt"));
    //part 1
    let res = count_visible(&trees);
    println!("Summary: {}", res);
    //part 2
    //let res = count_visible2(&trees);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| {
                    assert!(x.is_ascii_digit(), "not int");
                    b'9' - x as u8
                })
                .collect()
        })
        .collect()
}
fn count_visible(trees: &[Vec<u8>]) -> usize {
    let mut count = 0;
    for _ in trees.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let trees = parse(include_str!("../sample.txt"));
    dbg!(&trees);
    //part 1
    let res = count_visible(&trees);
    assert_eq!(res, 42);
    //part 2
    // let res = count_visible2(&trees);
    // assert_eq!(res, 42);
}
