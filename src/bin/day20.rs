use advent2022::*;
use std::collections::VecDeque;

fn main() {
    let numbers = parse(input!());
    //part 1
    let res = decrypt_groove_coord(&numbers);
    println!("Summary: {}", res);
    //part 2
    let res = decrypt_groove_coord2(&numbers);
    println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.parse().expect("not int")).collect()
}
fn decrypt_groove_coord(numbers: &[i64]) -> i64 {
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
fn decryption_key(numbers: &[i64]) -> Vec<i64> {
    numbers.iter().map(|n| n * 811589153).collect()
}
fn decrypt_groove_coord2(numbers: &[i64]) -> i64 {
    let numbers = decryption_key(numbers);
    let new = (0..10).fold(numbers.to_vec(), |acc, _| decrypt(&acc));
    // find 0 ...
    let (zero_pos, _) = new
        .iter()
        .enumerate()
        .find(|(_, v)| **v == 0)
        .expect("a zero");
    let l = new.len();
    new[(zero_pos + 1000) % l] + new[(zero_pos + 2000) % l] + new[(zero_pos + 3000) % l]
}
fn decrypt(numbers: &[i64]) -> Vec<i64> {
    let mut num: Vec<(i64, bool)> = numbers.iter().cloned().map(|n| (n, false)).collect();
    for _ in 0..num.len() {
        // Start again from beginning
        for i in 0_i64..(num.len() as i64) {
            let (n, moved) = *num.get(i as usize).expect("a number");
            if moved {
                continue;
            }
            let move_len = num.len() as i64 - 1;
            let nmove = n % (move_len);
            let mut new_pos = (i + nmove) % num.len() as i64;
            if new_pos < 0 {
                new_pos += move_len;
            }
            if new_pos < i && n > 0 {
                new_pos += 1;
            }
            if new_pos == 0 && new_pos != i {
                if n < 0 {
                    new_pos = move_len
                }
                if n > 0 {
                    new_pos = 1;
                }
            }
            num.iter().for_each(|(x, _)| {
                print!("{x}");
                if n == *x {
                    print!("*")
                }
                print!(" ")
            });
            println!();
            println!(
                "Moving {n} from {i} to {new_pos} ( ({n} % {move_len} = {nmove}) + {i} ) % {} = {} (after_adjust: {new_pos}",
                num.len() as i64,
                (i + nmove) % num.len() as i64
            );
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
    assert_eq!(decrypt(&numbers), vec![1, 2, -3, 4, 0, 3, -2]);
    let res = decrypt_groove_coord(&numbers);
    assert_eq!(res, 3);
    //part 2
    println!();
    assert_eq!(decrypt(&[3, 0, 1]), vec![0, 1, 3]);
    println!();
    assert_eq!(decrypt(&[0, 7, 1]), vec![0, 7, 1]);
    let numbers2 = decryption_key(&numbers);
    assert_eq!(
        numbers2,
        vec![
            811589153,
            1623178306,
            -2434767459,
            2434767459,
            -1623178306,
            0,
            3246356612
        ],
        "new numbers"
    );
    assert_eq!(
        (0..1).fold(numbers2.to_vec(), |acc, _| decrypt(&acc)),
        vec![
            0,
            -2434767459,
            3246356612,
            -1623178306,
            2434767459,
            1623178306,
            811589153
        ],
        "New numbers after 1 round"
    );
    assert_eq!(
        (0..2).fold(numbers2.to_vec(), |acc, _| decrypt(&acc)),
        vec![
            0,
            2434767459,
            1623178306,
            3246356612,
            -2434767459,
            -1623178306,
            811589153
        ],
        "after 2 rounds"
    );
    assert_eq!(
        (0..4).fold(numbers2.to_vec(), |acc, _| decrypt(&acc)),
        vec![
            0,
            1623178306,
            -2434767459,
            811589153,
            2434767459,
            3246356612,
            -1623178306
        ],
        "after 4 rounds"
    );
    assert_eq!(
        (0..10).fold(numbers2.to_vec(), |acc, _| decrypt(&acc)),
        vec![
            0,
            -2434767459,
            1623178306,
            3246356612,
            -1623178306,
            2434767459,
            811589153
        ],
        "after 10 rounds"
    );
    let res = decrypt_groove_coord2(&numbers);
    assert_eq!(res, 1623178306);
}
