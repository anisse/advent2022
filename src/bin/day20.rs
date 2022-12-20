use advent2022::*;
use std::collections::VecDeque;

fn main() {
    let numbers = parse(input!());
    //part 1
    let res = decrypt_groove_coord(&numbers);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&numbers);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse().expect("not int")).collect()
}
fn decrypt_groove_coord(numbers: &[i32]) -> i32 {
    let new = decrypt(numbers);
    // find 0 ...
    let (zero_pos, _) = new
        .iter()
        .enumerate()
        .find(|(_, v)| **v == 0)
        .expect("a zero");
    let l = new.len();
    new[(zero_pos + 1000) % l] + new[(zero_pos + 2000) % l] + new[(zero_pos + 3000) % l]
}
fn decrypt(numbers: &[i32]) -> Vec<i32> {
    let mut num: Vec<(i32, bool)> = numbers.iter().cloned().map(|n| (n, false)).collect();
    for _ in 0..num.len() {
        // Start again from beginning
        for i in 0_i32..(num.len() as i32) {
            let (n, moved) = *num.get(i as usize).expect("a number");
            if moved {
                continue;
            }
            //let n = num.get(i).expect("a number");
            let mut new_pos = (i + n) % (num.len() as i32 - 1);
            match new_pos.cmp(&0) {
                std::cmp::Ordering::Less => new_pos += num.len() as i32 - 1,
                std::cmp::Ordering::Equal => new_pos = num.len() as i32 - 1,
                std::cmp::Ordering::Greater => {}
            }
            //println!("Moving {n} from {i} to {new_pos} (length: {})", num.len());
            num.remove(i as usize);
            num.insert(new_pos as usize, (n, true));
            break;
        }
    }
    num.iter()
        .map(|(a, b)| {
            assert!(b);
            *a
        })
        .collect()
}

#[test]
fn test() {
    let numbers = parse(sample!());
    //part 1
    assert_eq!(decrypt(&numbers), vec![1, 2, -3, 4, 0, 3, -2]);
    let res = decrypt_groove_coord(&numbers);
    assert_eq!(res, 3);
    //part 2
    // let res = operation2(&numbers);
    // assert_eq!(res, 42);
}
