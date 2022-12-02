fn main() {
    let strategy_guide = parse(include_str!("../input.txt"));
    //part 1
    let score = compute_score(&strategy_guide);
    println!("Score: {}", score);
    //part 2
    //let score = operation2(&strategy_guide);
    //println!("Summary2: {}", score);
}

use std::cmp::Ordering;

use crate::Play::*;

#[derive(PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}
impl std::cmp::Ord for Play {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Rock => match other {
                Rock => Ordering::Equal,
                Paper => Ordering::Less,
                Scissors => Ordering::Greater,
            },
            Paper => match other {
                Rock => Ordering::Greater,
                Paper => Ordering::Equal,
                Scissors => Ordering::Less,
            },
            Scissors => match other {
                Rock => Ordering::Less,
                Paper => Ordering::Greater,
                Scissors => Ordering::Equal,
            },
        }
    }
}
impl std::cmp::PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}
impl FromIterator<Play> for (Play, Play) {
    fn from_iter<T: IntoIterator<Item = Play>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        let one = it.next().expect("No first element");
        let two = it.next().expect("No second element");
        (one, two)
    }
}
fn parse(input: &str) -> Vec<(Play, Play)> {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|p| {
                    assert!(p.len() == 1, "Len should be 1");
                    match p.chars().next().unwrap() {
                        'A' | 'X' => Rock,
                        'B' | 'Y' => Paper,
                        'C' | 'Z' => Scissors,
                        _ => panic!("Unexpected char"),
                    }
                })
                .collect()
        })
        .collect()
}
fn compute_score(strategy_guide: &[(Play, Play)]) -> usize {
    let mut score = 0;
    for (opponent, i) in strategy_guide.iter() {
        score += match i.cmp(opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };
        score += match i {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
    }
    score
}

#[test]
fn test() {
    let strategy_guide = parse(include_str!("../sample.txt"));
    //part 1
    let score = compute_score(&strategy_guide);
    assert_eq!(score, 15);
    //part 2
    // let score = operation2(&strategy_guide);
    // assert_eq!(score, 42);
}
