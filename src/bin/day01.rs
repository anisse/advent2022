use advent2022::*;

fn main() {
    let elfcalories = parse(input!());
    //part 1
    let res = max_calories(elfcalories.clone());
    println!("Max calories: {}", res);
    //part 2
    let res = top3_max(elfcalories);
    println!("Top 3: {}", res);
}

type ParsedItem = Vec<usize>;

fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + '_ + Clone {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|x| x.parse().expect("not int")).collect())
}
fn max_calories<I>(elfcalories: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    elfcalories.map(|v| v.iter().sum()).max().expect("no max")
}

fn top3_max<I>(elfcalories: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut sorted_sum = elfcalories.map(|v| v.iter().sum()).collect::<Vec<usize>>();
    sorted_sum.sort_by(|a, b| b.cmp(a));
    sorted_sum.iter().take(3).sum()
}
#[test]
fn test() {
    let mut elfcalories = parse(sample!());
    //part 1
    let res = max_calories(elfcalories.clone());
    assert_eq!(res, 24000);
    //part 2
    let res = top3_max(&mut elfcalories);
    assert_eq!(res, 24000 + 11000 + 10000);
}
