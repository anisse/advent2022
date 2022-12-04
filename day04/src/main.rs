use std::ops::RangeInclusive;

fn main() {
    let sections = parse(include_str!("../input.txt"));
    //part 1
    let res = count_overlap(&sections);
    println!("Summary: {}", res);
    //part 2
    let res = count_overlap_full(&sections);
    println!("Summary2: {}", res);
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

fn range_overlap(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    (range1.start() <= range2.end() && range1.start() >= range2.start())
        || (range1.end() <= range2.end() && range1.end() >= range2.end())
        || (range2.start() <= range1.end() && range2.start() >= range1.start())
        || (range2.end() <= range1.end() && range2.end() >= range1.end())
}

fn count_overlap_full(sections: &[[RangeInclusive<u32>; 2]]) -> usize {
    sections
        .iter()
        .filter(|ranges| range_overlap(&ranges[0], &ranges[1]))
        .count()
}

#[test]
fn test() {
    let sections = parse(include_str!("../sample.txt"));
    //part 1
    let res = count_overlap(&sections);
    assert_eq!(res, 2);
    //part 2
    let res = count_overlap_full(&sections);
    assert_eq!(res, 4);
}
