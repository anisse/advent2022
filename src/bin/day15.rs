use std::cmp::{max, min};

use advent2022::*;
fn main() {
    let coords = parse(input!());
    //part 1
    let res = empty_pos_at(&coords, 2000000);
    println!("Summary: {}", res);
    //part 2
    //let res = empty_pos_at2(&coords);
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

fn empty_pos_at(coords: &[[Pos; 2]], y: i64) -> usize {
    let (min_p, max_p) = map_coords(coords);
    let map_width = max_p.x - min_p.x + 1;
    dbg!(&min_p);
    dbg!(&max_p);
    dbg!(&map_width);
    //let map_height = max_p.y - min_p.y;
    let mut line = vec![false; map_width as usize];
    coords
        .iter()
        .for_each(|sb| fill_line(&sb[0], &sb[1], y, &mut line, min_p.x));
    line.iter()
        .for_each(|b| if *b { print!("#") } else { print!(".") });
    println!();
    line.iter().filter(|b| **b).count()
}

fn fill_line(sensor: &Pos, beacon: &Pos, y: i64, line: &mut [bool], min_x: i64) {
    let md = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
    /*
        if ((sensor.y - md)..=(sensor.y + md)).contains(&y) {
            //do the filling
        }
    */
    let remaining = md - (sensor.y - y).abs();
    ((sensor.x - remaining)..(sensor.x + remaining)).for_each(|x| {
        let coord_x = (x - min_x) as usize;
        if (0..line.len()).contains(&coord_x) {
            line[(x - min_x) as usize] = true
        }
    });
}

fn map_coords(coords: &[[Pos; 2]]) -> (Pos, Pos) {
    coords
        .iter()
        .flat_map(|sb| {
            let md = (sb[0].x - sb[1].x).abs() + (sb[0].y - sb[1].y).abs();
            vec![
                Pos {
                    x: sb[0].x + md,
                    y: sb[0].y,
                },
                Pos {
                    x: sb[0].x - md,
                    y: sb[0].y,
                },
                Pos {
                    x: sb[0].x,
                    y: sb[0].y + md,
                },
                Pos {
                    x: sb[0].x,
                    y: sb[0].y - md,
                },
            ]
        })
        //.flatten()
        .fold(
            (
                Pos {
                    x: i64::MAX,
                    y: i64::MAX,
                },
                Pos {
                    x: i64::MIN,
                    y: i64::MIN,
                },
            ),
            |mut acc, p| {
                acc.0.x = min(p.x, acc.0.x);
                acc.0.y = min(p.y, acc.0.y);
                acc.1.x = max(p.x, acc.1.x);
                acc.1.y = max(p.y, acc.1.y);
                acc
            },
        )
}

#[test]
fn test() {
    let coords = parse(sample!());
    //part 1
    let res = empty_pos_at(&coords, 10);
    assert_eq!(res, 26);
    //part 2
    // let res = empty_pos_at2(&coords);
    // assert_eq!(res, 42);
}
