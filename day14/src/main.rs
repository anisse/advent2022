fn main() {
    let rocklines = parse(include_str!("../input.txt"));
    //part 1
    let res = max_caught_sand(&rocklines);
    println!("Summary: {}", res);
    //part 2
    //let res = max_caught_sand2(&rocklines);
    //println!("Summary2: {}", res);
}

struct Pos {
    x: u16,
    y: u16,
}

type Line = Vec<Pos>;

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|p| {
                    let mut pi = p.split(',');
                    Pos {
                        x: pi.next().expect("no x").parse().expect("not int"),
                        y: pi.next().expect("no y").parse().expect("not int"),
                    }
                })
                .collect()
        })
        .collect()
}
fn max_caught_sand(rocklines: &[Line]) -> usize {
    let mut count = 0;
    for _ in rocklines.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let rocklines = parse(include_str!("../sample.txt"));
    //part 1
    let res = max_caught_sand(&rocklines);
    assert_eq!(res, 24);
    //part 2
    // let res = max_caught_sand2(&rocklines);
    // assert_eq!(res, 42);
}
