fn main() {
    let rucksacks = parse(include_str!("../input.txt"));
    //part 1
    let res = prio_sum(&rucksacks);
    println!("Sum: {}", res);
    //part 2
    let res = prio_badges(&rucksacks);
    println!("Badge sum: {}", res);
}
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect()).collect()
}
fn letter_to_score(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize,
        'A'..='Z' => c as usize - 'A' as usize + 26,
        _ => panic!("Unexpected char {}", c),
    }
}

fn to_map(sack: &[char]) -> Vec<bool> {
    let mut map = vec![false; 26 * 2];
    for c in sack.iter() {
        map[letter_to_score(*c)] = true;
    }
    map
}
fn prio_sum(rucksacks: &[Vec<char>]) -> usize {
    rucksacks
        .iter()
        .map(|s| {
            let mid = s.len() / 2;
            let lmap = to_map(&s[..mid]);
            let rmap = to_map(&s[mid..]);
            lmap.iter()
                .zip(rmap.iter())
                .enumerate()
                .find(|(_, (a, b))| **a && **b)
                .map_or(0, |(i, (_, _))| i + 1)
        })
        .sum()
}

fn prio_badges(rucksacks: &[Vec<char>]) -> usize {
    rucksacks
        .chunks(3)
        .map(|group| {
            let g1m = to_map(group.get(0).expect("no first group element"));
            let g2m = to_map(group.get(1).expect("no second group element"));
            let g3m = to_map(group.get(2).expect("no third group element"));
            g1m.iter()
                .zip(g2m.iter())
                .zip(g3m.iter())
                .enumerate()
                .find(|(_, ((a, b), c))| **a && **b && **c)
                .map_or(0, |(i, ((_, _), _))| i + 1)
        })
        .sum()
}

#[test]
fn test() {
    let rucksacks = parse(include_str!("../sample.txt"));
    //part 1
    let res = prio_sum(&rucksacks);
    assert_eq!(res, 157);
    //part 2
    let res = prio_badges(&rucksacks);
    assert_eq!(res, 70);
}
