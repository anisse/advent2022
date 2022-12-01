fn main() {
    let elfcalories = parse(include_str!("../input.txt"));
    //part 1
    let res = max_calories(&elfcalories);
    println!("Max calories: {}", res);
    //part 2
    let res = top3_max(&elfcalories);
    println!("Top 3: {}", res);
}
fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|x| x.parse().expect("not int")).collect())
        .collect()
}
fn max_calories(elfcalories: &[Vec<usize>]) -> usize {
    elfcalories
        .iter()
        .map(|v| v.iter().sum())
        .max()
        .expect("no max")
}

fn top3_max(elfcalories: &[Vec<usize>]) -> usize {
    let mut sorted_sum = elfcalories
        .iter()
        .map(|v| v.iter().sum())
        .collect::<Vec<usize>>();
    sorted_sum.sort_by(|a, b| b.cmp(a));
    sorted_sum.iter().take(3).sum()
}
#[test]
fn test() {
    let elfcalories = parse(include_str!("../sample.txt"));
    //part 1
    let res = max_calories(&elfcalories);
    assert_eq!(res, 24000);
    //part 2
    let res = top3_max(&elfcalories);
    assert_eq!(res, 24000 + 11000 + 10000);
}
