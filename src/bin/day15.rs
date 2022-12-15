use advent2022::*;
fn main() {
    let coords = parse(input!());
    //part 1
    let res = closest_at(&coords);
    println!("Summary: {}", res);
    //part 2
    //let res = closest_at2(&coords);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<[Pos; 2]> {
    input
        .lines()
        .map(|l| {
            let mut li = l.split_ascii_whitespace();
            let x1 = li
                .nth(2)
                .expect("no x1")
                .split('=')
                .nth(1)
                .expect("no x1 val")
                .split(',')
                .next()
                .expect("no x1 val int")
                .parse()
                .expect("not int");
            let y1 = li
                .next()
                .expect("no y1")
                .split('=')
                .nth(1)
                .expect("no y1 val")
                .split(':')
                .next()
                .expect("no y1 val int")
                .parse()
                .expect("not int");
            let x2 = li
                .nth(4)
                .expect("no x2")
                .split('=')
                .nth(1)
                .expect("no x2 val")
                .strip_suffix(',')
                .expect("no , to remove")
                .parse()
                .expect("not int x2");
            let y2 = li
                .next()
                .expect("no y2")
                .split('=')
                .nth(1)
                .expect("no y2 val")
                .parse()
                .expect("not int");

            [Pos { x: x1, y: y1 }, Pos { x: x2, y: y2 }]
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

fn closest_at(coords: &[[Pos; 2]]) -> usize {
    let mut count = 0;
    for _ in coords.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let coords = parse(sample!());
    //part 1
    let res = closest_at(&coords);
    assert_eq!(res, 42);
    //part 2
    // let res = closest_at2(&coords);
    // assert_eq!(res, 42);
}
