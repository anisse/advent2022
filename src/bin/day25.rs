use advent2022::*;
fn main() {
    let snafus = parse(input!());
    //part 1
    let res = sum(&snafus);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&snafus);
    //println!("Summary2: {}", res);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Snafu {
    num: i64,
}
impl From<&str> for Snafu {
    fn from(s: &str) -> Self {
        let mut num = 0;
        for (i, c) in s.bytes().rev().enumerate() {
            match c {
                b'0'..=b'2' => num += 5_i64.pow(i as u32) * (c as i64 - b'0' as i64),
                b'-' => num -= 5_i64.pow(i as u32),
                b'=' => num -= 2 * 5_i64.pow(i as u32),
                _ => panic!("Unknown char in Snafu"),
            }
        }
        Self { num }
    }
}
impl From<Snafu> for i64 {
    fn from(value: Snafu) -> Self {
        value.num
    }
}
fn parse(input: &str) -> Vec<Snafu> {
    input.lines().map(|x| x.into()).collect()
}
fn sum(snafus: &[Snafu]) -> i64 {
    snafus.iter().map(|s| i64::from(*s)).sum()
}

#[test]
fn test_convert() {
    let nums = [
        (1_i64, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ];
    for (i, s) in nums.iter() {
        assert_eq!(i64::from(Snafu::from(*s)), *i, "{s} is not equal to {i}");
    }
}

#[test]
fn test() {
    let snafus = parse(sample!());
    //part 1
    let res = sum(&snafus);
    assert_eq!(res, 4890);
    //part 2
    // let res = operation2(&snafus);
    // assert_eq!(res, 42);
}
