use advent2022::*;
fn main() {
    let cubes = parse(input!());
    //part 1
    let res = operation(&cubes);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&cubes);
    //println!("Summary2: {}", res);
}

type Cube = Vec<u8>;

fn parse(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().expect("not int")).collect())
        .collect()
}
fn operation(cubes: &[Cube]) -> usize {
    let mut count = 0;
    for _ in cubes.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let cubes = parse(sample!());
    //part 1
    let res = operation(&cubes);
    assert_eq!(res, 64);
    //part 2
    // let res = operation2(&cubes);
    // assert_eq!(res, 42);
}
