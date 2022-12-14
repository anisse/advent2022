use advent2022::*;

fn main() {
    let numbers = parse(input!());
    //part 1
    let res = decrypt_groove_coord(&numbers);
    println!("Groove coord after 1 round: {}", res);
    //part 2
    let res = decrypt_groove_coord2(&numbers);
    println!("Groove coord after mix + 10 round: {}", res);
}
fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.parse().expect("not int")).collect()
}
fn decrypt_groove_coord(numbers: &[i64]) -> i64 {
    let new = decrypt_rounds(numbers, 1);
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
    let new = decrypt_rounds(&numbers, 10);
    // find 0 ...
    let (zero_pos, _) = new
        .iter()
        .enumerate()
        .find(|(_, v)| **v == 0)
        .expect("a zero");
    let l = new.len();
    new[(zero_pos + 1000) % l] + new[(zero_pos + 2000) % l] + new[(zero_pos + 3000) % l]
}
fn decrypt_rounds(numbers: &[i64], rounds: usize) -> Vec<i64> {
    let mut num = numbers.to_vec();
    let mut order: Vec<_> = (0..numbers.len()).collect();
    for _r in 0..rounds {
        //println!("\nAt round {_r}, order is {order:?}\n");
        for k in 0..num.len() {
            // Start again from beginning
            for i in 0..(order.len() as i64) {
                let o = *order.get(i as usize).expect("a number");
                if k != o {
                    continue;
                }
                let n = *num.get(i as usize).expect("a number");
                let move_len = num.len() as i64 - 1;
                let mut nmove = n % (move_len);
                let mut new_pos = i;
                // equivalent to new_pos != i
                if nmove != 0 {
                    if nmove < 0 {
                        nmove += move_len;
                    }
                    if i + nmove > move_len {
                        nmove += 1;
                    }
                    new_pos = (i + nmove) % num.len() as i64;
                }
                if i == 0 && n != 0 && nmove == 0 {
                    new_pos = move_len;
                }
                // The implemenation below does not pass tests but it is *also* correct
                // Tests do not take into account the fact that the list is circular, so it does
                // not matter where the "start" of the flat view is. A proper way to test it would
                // be to only start at 0 (but it wouldn't allow multiple 0, like we do)
                /*
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
                */
                /*
                num.iter().for_each(|x| {
                    print!("{x}");
                    if n == *x {
                        print!("*")
                    }
                    print!(" ")
                });
                println!();
                println!(
                    "Moving {n} from {i} to {new_pos} \
                ({n} % {move_len} = {} -> {nmove} ; ({i} + {nmove} % {} = {new_pos})",
                    n % move_len,
                    num.len()
                );
                */
                num.remove(i as usize);
                num.insert(new_pos as usize, n);
                order.remove(i as usize);
                order.insert(new_pos as usize, k);
                break;
            }
        }
    }
    num
}

#[test]
fn test() {
    let numbers = parse(sample!());
    //part 1
    assert_eq!(decrypt_rounds(&numbers, 1), vec![1, 2, -3, 4, 0, 3, -2]);
    assert_eq!(decrypt_rounds(&numbers, 1), vec![1, 2, -3, 4, 0, 3, -2]);
    let res = decrypt_groove_coord(&numbers);
    assert_eq!(res, 3);
    //part 2
    println!();
    assert_eq!(decrypt_rounds(&[3, 0, 1], 1), vec![0, 1, 3]);
    println!();
    assert_eq!(decrypt_rounds(&[0, 7, 1], 1), vec![0, 7, 1]);
    println!();
    assert_eq!(decrypt_rounds(&[-3, 0, 1], 1), vec![0, 1, -3]);
    println!();
    assert_eq!(decrypt_rounds(&[0, -7, 1], 1), vec![0, -7, 1]);
    println!();
    assert_eq!(decrypt_rounds(&[0, 0, -1], 1), vec![0, -1, 0]);
    println!();
    assert_eq!(decrypt_rounds(&[0, 0, 1], 1), vec![0, 1, 0]);
    println!();
    assert_eq!(decrypt_rounds(&[0, 0, 3], 1), vec![0, 3, 0]);
    println!();
    assert_eq!(decrypt_rounds(&[0, 0, -3], 1), vec![0, -3, 0]);
    println!();
    assert_eq!(decrypt_rounds(&[-1, 0, 0], 1), vec![0, -1, 0]);
    println!();
    assert_eq!(decrypt_rounds(&[-2, 0, 0], 1), vec![0, 0, -2]);
    println!();
    assert_eq!(decrypt_rounds(&[-4, 0, 0], 1), vec![0, 0, -4]);
    println!();
    assert_eq!(decrypt_rounds(&[-2, 0, 0, 0], 1), vec![0, -2, 0, 0]);
    println!();
    assert_eq!(decrypt_rounds(&[-3, 0, 0, 0], 1), vec![0, 0, 0, -3]);
    println!();
    assert_eq!(decrypt_rounds(&[2, 0, 0], 1), vec![0, 0, 2]);
    println!();
    assert_eq!(decrypt_rounds(&[2, 0, 0, 0], 1), vec![0, 0, 2, 0]);
    println!();
    assert_eq!(decrypt_rounds(&[3, 0, 0, 0], 1), vec![0, 0, 0, 3]);
    println!();
    assert_eq!(decrypt_rounds(&[4, 0, 0, 0], 1), vec![0, 4, 0, 0]);
    println!();
    assert_eq!(decrypt_rounds(&[3, 0, 0], 1), vec![0, 3, 0]);
    println!();
    assert_eq!(decrypt_rounds(&[-3, 0, 0], 1), vec![0, -3, 0]);
    println!();
    assert_eq!(decrypt_rounds(&[0, 0, 0, 3], 1), vec![0, 0, 0, 3]);
    println!();
    assert_eq!(decrypt_rounds(&[0, 0, 0, 6], 1), vec![0, 0, 0, 6]);
    println!();
    assert_eq!(decrypt_rounds(&[0, 0, 0, -6], 1), vec![0, 0, 0, -6]);
    println!();
    assert_eq!(decrypt_rounds(&[0, 0, 0, 9], 1), vec![0, 0, 0, 9]);
    println!();
    assert_eq!(decrypt_rounds(&[0, 0, 0, 10], 1), vec![0, 10, 0, 0]);
    println!();
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
        decrypt_rounds(&numbers2, 1),
        decrypt_rounds(&numbers2, 1),
        "Rounds vs base implementation"
    );
    assert_eq!(
        decrypt_rounds(&numbers2, 1),
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
        decrypt_rounds(&numbers2, 2),
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
        decrypt_rounds(&numbers2, 4),
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
        decrypt_rounds(&numbers2, 10),
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
