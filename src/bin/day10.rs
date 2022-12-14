use advent2022::*;
fn main() {
    //part 1
    let res = signal_strength_simple(input!());
    println!("Signal strength: {}", res);
    //part 2
    let res = crt_simple(input!());
    println!("CRT:\n{}", res);
}

fn parse(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.lines().flat_map(|x| match &x[..4] {
        "noop" => vec![0_i32],
        "addx" => vec![0, x[5..].parse().expect("not int")],
        _ => panic!("not instruction"),
    })
}

fn signal_strength_simple(input: &str) -> i32 {
    parse(input)
        .scan(1, |x, add| {
            *x += add;
            Some(*x)
        })
        .enumerate()
        .map(|(i, x)| (i + 2, x))
        /*
        .filter(|(i, x)| {
            println!("{i}: {x}");
            true
        })
        */
        .filter(|(i, _)| *i >= 20 && ((*i - 20) % 40) == 0)
        .map(|(i, x)| (i as i32) * x)
        .sum()
}

fn crt_simple(input: &str) -> String {
    [0].into_iter()
        .chain(parse(input))
        .scan(1, |x, add| {
            *x += add;
            Some(*x)
        })
        .enumerate()
        .map(|(i, x)| (i, (x - 1..=x + 1).contains(&(i as i32 % 40))))
        .map(|(i, val)| (i, if val { '#' } else { '.' }))
        .flat_map(|(i, val)| {
            if i > 1 && i % 40 == 0 {
                vec!['\n', val]
            } else {
                vec![val]
            }
        })
        .take(41 * 6)
        .collect()
}

#[test]
fn test() {
    //part 1 simple
    let res = signal_strength_simple(sample!());
    assert_eq!(res, 13140, "simple version");
    //part 2
    let res = crt_simple(sample!());
    assert_eq!(
        res,
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
    );
}
