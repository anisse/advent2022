use std::{
    cmp::{max, min},
    ops::Range,
};

use advent2022::*;
fn main() {
    let coords = parse(input!());
    //part 1
    let res = empty_pos_at(&coords, 2000000);
    println!("Summary: {}", res);
    //part 2
    let res = empty_pos_beacon(&coords, 4000000);
    println!("Summary2: {}", res);
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

fn empty_pos_line(coords: &[[Pos; 2]], y: i64) -> (Vec<bool>, i64) {
    let (min_p, max_p) = map_coords(coords);
    let map_width = max_p.x - min_p.x + 1;
    /*
    dbg!(&min_p);
    dbg!(&max_p);
    dbg!(&map_width);
    */
    //let map_height = max_p.y - min_p.y;
    let mut line = vec![false; map_width as usize];
    coords
        .iter()
        .for_each(|sb| fill_line(&sb[0], &sb[1], y, &mut line, min_p.x));
    remove_beacons(coords, y, &mut line, min_p.x);
    (line, min_p.x)
}
fn empty_pos_at(coords: &[[Pos; 2]], y: i64) -> usize {
    let (line, _) = empty_pos_line(coords, y);
    /*
    line.iter()
        .for_each(|b| if *b { print!("#") } else { print!(".") });
    println!();
    */
    line.iter().filter(|b| **b).count()
}

fn fill_line(sensor: &Pos, beacon: &Pos, y: i64, line: &mut [bool], min_x: i64) {
    let md = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
    /*
        if ((sensor.y - md)..=(sensor.y + md)).contains(&y) {
            //do the filling
        }
    */
    let r = range_at(sensor, beacon, y);
    r.for_each(|x| {
        let coord_x = (x - min_x) as usize;
        if (0..line.len()).contains(&coord_x) {
            line[(x - min_x) as usize] = true
        }
    });
    // Now remove beacons and sensor coords
}

fn remove_beacons(coords: &[[Pos; 2]], y: i64, line: &mut [bool], min_x: i64) {
    coords.iter().map(|[_, b]| b).for_each(|p| {
        if y == p.y {
            line[(p.x - min_x) as usize] = false;
        }
    });
}

fn render_full_map(coords: &[[Pos; 2]]) {
    let (min_p, max_p) = map_coords(coords);
    let map_width = max_p.x - min_p.x + 1;
    print!("   ");
    (min_p.x..=max_p.x).for_each(|x| print!("{}", (x % 10).abs()));
    println!();
    (min_p.y..(max_p.y + 1)).for_each(|y| {
        let mut line = vec![false; map_width as usize];
        coords
            .iter()
            .for_each(|sb| fill_line(&sb[0], &sb[1], y, &mut line, min_p.x));
        print!("{:3}", y);
        line.iter().enumerate().for_each(|(x, c)| {
            for [s, b] in coords.iter() {
                if s.x - min_p.x == x as i64 && s.y == y {
                    print!("S");
                    return;
                }
                if b.x - min_p.x == x as i64 && b.y == y {
                    print!("B");
                    return;
                }
            }
            if *c {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
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

fn range_at(sensor: &Pos, beacon: &Pos, y: i64) -> Range<i64> {
    let md = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
    let remaining = md - (sensor.y - y).abs();
    (sensor.x - remaining)..(sensor.x + remaining + 1)
}
fn line_has_empty_slots(coords: &[[Pos; 2]], y: i64, min_x: i64, max_x: i64) -> bool {
    #[allow(clippy::reversed_empty_ranges)]
    let mut ranges: Vec<_> = coords.iter().map(|[s, b]| range_at(s, b, y)).collect();
    let mut range = ranges.pop().expect("something");
    'outer: loop {
        //println!("Merged range is {range:?}");
        let l = ranges.len();
        for i in 0..ranges.len() {
            //println!("At {i} of {} (l={l}): {:?}", ranges.len(), ranges[i]);
            if ranges[i].start < ranges[i].end {
                if range.start < range.end {
                    if range.contains(&(ranges[i].start - 1))
                        || range.contains(&(ranges[i].end))
                        || ranges[i].contains(&(range.start - 1))
                        || ranges[i].contains(&(range.end))
                    {
                        let r1 = ranges.swap_remove(i);
                        range.start = min(r1.start, range.start);
                        range.end = max(r1.end, range.end);
                        continue 'outer;
                    }
                } else {
                    let r1 = ranges.swap_remove(i);
                    //println!("Swapping {r1:?} and {range:?}");
                    ranges.push(range.start..range.end);
                    range.start = r1.start;
                    range.end = r1.end;
                    continue 'outer;
                }
            }
        }
        if l == ranges.len() {
            break;
        }
    }
    /*
    println!(
        "Got merged range of {range:?} start {min_x} is in:{} end {max_x} is in: {}",
        range.contains(&min_x),
        range.contains(&max_x),
    );
    */
    !range.contains(&min_x) || !range.contains(&max_x)
}
fn empty_pos_line_bounded(coords: &[[Pos; 2]], y: i64, min: i64, max: i64) -> Vec<bool> {
    let mut line = vec![false; (max - min) as usize];
    coords
        .iter()
        .for_each(|sb| fill_line(&sb[0], &sb[1], y, &mut line, min));
    remove_beacons(coords, y, &mut line, min);
    line
}
fn empty_pos_beacon(coords: &[[Pos; 2]], max_xy: i64) -> usize {
    let (min_p, max_p) = map_coords(coords);
    let real_max_x = min(max_p.x, max_xy);
    let real_max_y = min(max_p.y, max_xy);
    let real_min_x = max(min_p.x, 0);
    let real_min_y = max(min_p.y, 0);
    for y in real_min_y..=real_max_y {
        if y % 100000 == 0 {
            println!("iterating at {y}");
        }
        if line_has_empty_slots(coords, y, real_min_x, real_max_x) {
            //println!("Line {y} has empty slots");
            let line = empty_pos_line_bounded(coords, y, real_min_x, real_max_x);
            let count = line.iter().filter(|b| !**b).count();
            /*
            line.iter()
                .for_each(|b| if *b { print!("#") } else { print!(".") });
            println!();
            println!("Line {y}: (from min: {real_min_x}) count is {count}");
            */
            if count == 1 {
                let x = line
                    .iter()
                    .enumerate()
                    .find(|(_, b)| !**b)
                    .map(|(i, _)| i)
                    .unwrap() as i64
                    - real_min_x;
                println!("Got {x}x{y}");
                return (x as usize) * 4000000 + y as usize;
            }
        }
    }
    unreachable!()
}

#[test]
fn test() {
    let coords = parse(sample!());
    render_full_map(&coords);
    //part 1
    let res = empty_pos_at(&coords, 10);
    assert_eq!(res, 26);
    //part 2
    let res = empty_pos_beacon(&coords, 20);
    assert_eq!(res, 56000011);
}
