use std::ops::RangeInclusive;

fn main() {
    let sections = parse(include_str!("../input.txt"));
    //part 1
    let res = count_overlap(&sections);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&sections);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input.lines().map(|x| x.parse().expect("not int")).collect()
}
fn count_overlap(sections: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    let mut count = 0;
    for _ in sections.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let sections = parse(include_str!("../sample.txt"));
    //part 1
    let res = count_overlap(&sections);
    assert_eq!(res, 2);
    //part 2
    // let res = operation2(&sections);
    // assert_eq!(res, 42);
}
