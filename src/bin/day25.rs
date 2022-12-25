use advent2022::*;
fn main() {
    let snafus = parse(input!());
    //part 1
    let res = sum(&snafus);
    println!("Here it is: {}", Snafu::str(res));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Snafu {
    num: i64,
}
impl Snafu {
    fn str(num: i64) -> String {
        Snafu { num }.into()
    }
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
impl From<Snafu> for String {
    fn from(value: Snafu) -> Self {
        let mut num = value.num;
        let mut s: Vec<char> = Vec::new();
        let mut pow = 0;
        let mut consumed;
        //println!("From called for {value:?}");
        while num != 0 {
            let i = num % 5_i64.pow(pow + 1);
            let c = i / 5_i64.pow(pow);
            match c {
                0..=2 => {
                    s.push((b'0' + c as u8) as char);
                    consumed = c;
                }
                3 => {
                    s.push('=');
                    consumed = -2;
                }
                4 => {
                    s.push('-');
                    consumed = -1;
                }
                _ => unreachable!(),
            }
            //println!("Treating value.num: at pow {pow}: i={i}, c={c}, num={num}, val={val}");
            num -= consumed * 5_i64.pow(pow);
            pow += 1;
        }
        s.into_iter().rev().collect()
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
    for (i, s) in nums.iter() {
        let sna = String::from(Snafu { num: *i });
        assert_eq!(
            String::from(Snafu { num: *i }),
            s.to_string(),
            "'{sna}' ({i}) is not equal to {s}"
        );
    }
}

#[test]
fn test() {
    let snafus = parse(sample!());
    //part 1
    let res = sum(&snafus);
    assert_eq!(res, 4890);
}
