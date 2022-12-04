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
fn parse(input: &str) -> Vec<[RangeInclusive<u32>; 2]> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|r| {
                    let v: Vec<u32> = r
                        .split('-')
                        .map(|i| i.parse::<u32>().expect("not int"))
                        .collect();
                    *v.first().expect("No first element")..=*v.get(1).expect("No second element")
                })
                .collect::<Vec<_>>()
                .try_into()
                .expect("Not two elements")
        })
        .collect()
}
fn count_overlap(sections: &[[RangeInclusive<u32>; 2]]) -> usize {
    sections
        .iter()
        .filter(|ranges| {
            (ranges[0].contains(ranges[1].start()) && ranges[0].contains(ranges[1].end()))
                || (ranges[1].contains(ranges[0].start()) && ranges[1].contains(ranges[0].end()))
        })
        .count()
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
