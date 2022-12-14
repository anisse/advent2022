use std::cmp::{max, min};

use crate::Element::*;

fn main() {
    let rocklines = parse(include_str!("../input.txt"));
    //part 1
    let res = max_caught_sand(&rocklines);
    println!("Grains of sand to go out of map: {}", res);
    //part 2
    let res = max_caught_sand_inf(&rocklines);
    println!("Grains of sand to reach start of infinite map: {}", res);
}

#[derive(Debug, Clone)]
struct Pos {
    x: i16,
    y: i16,
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
    let mut cave = build_map(rocklines);
    //println!("{}", cave);
    let mut count = 0;
    loop {
        if add_grain(&mut cave) {
            return count;
        }
        count += 1
    }
}
fn max_caught_sand_inf(rocklines: &[Line]) -> usize {
    let mut cave = build_map_infinite(rocklines);
    //println!("{}", cave);
    let mut count = 0;
    loop {
        add_grain(&mut cave);
        count += 1;
        if let Sand = cave.grid[cave.start.y as usize][cave.start.x as usize] {
            return count;
        }
    }
}

fn add_grain(cave: &mut Cave) -> bool {
    let mut sand_pos = cave.start.clone();

    'outer: loop {
        for new_pos in [
            Pos {
                x: sand_pos.x,
                y: sand_pos.y + 1,
            },
            Pos {
                x: sand_pos.x - 1,
                y: sand_pos.y + 1,
            },
            Pos {
                x: sand_pos.x + 1,
                y: sand_pos.y + 1,
            },
        ]
        .iter()
        {
            // first check validity
            if new_pos.x < 0
                || new_pos.x >= cave.grid[0].len() as i16
                || new_pos.y >= cave.grid.len() as i16
            {
                //overflow
                return true;
            }
            match cave.grid[new_pos.y as usize][new_pos.x as usize] {
                Empty => {
                    sand_pos = new_pos.clone();
                    continue 'outer;
                }
                Rock | Sand => continue,
            }
        }
        cave.grid[sand_pos.y as usize][sand_pos.x as usize] = Sand;
        return false;
    }
}

#[derive(Debug, Clone)]
enum Element {
    Empty,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Cave {
    grid: Vec<Vec<Element>>,
    start: Pos,
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, l) in self.grid.iter().enumerate() {
            for (x, e) in l.iter().enumerate() {
                if self.start.x as usize == x && self.start.y as usize == y {
                    write!(f, "+")?;
                } else {
                    match e {
                        Empty => write!(f, ".")?,
                        Rock => write!(f, "#")?,
                        Sand => write!(f, "o")?,
                    };
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn build_map(rocklines: &[Line]) -> Cave {
    let (min_x, max_x, min_y, max_y) = rocklines
        .iter()
        .flatten()
        .chain([Pos { x: 500, y: 0 }].iter())
        .fold((i16::MAX, 0, i16::MAX, 0), |acc, p| {
            (
                min(acc.0, p.x),
                max(acc.1, p.x),
                min(acc.2, p.y),
                max(acc.3, p.y),
            )
        });
    //println!("max: {max_x}x{max_y}, min: {min_x}x{min_y}");
    let width = (max_x - min_x) as usize + 1;
    let height = (max_y - min_y) as usize + 1;
    /*
    println!(
        "making grid of {width}x{height}, start is at {}x{}",
        500 - min_x,
        0 - min_y
    );
    */
    let mut grid = vec![vec![Empty; width]; height];
    rocklines.iter().for_each(|l| {
        l.windows(2).for_each(|line| {
            //println!("Line {line:?}");
            if line[0].x == line[1].x {
                (min(line[0].y, line[1].y)..=max(line[0].y, line[1].y))
                    .map(|y| ((line[0].x - min_x) as usize, (y - min_y) as usize))
                    .for_each(|(x, y)| {
                        grid[y][x] = Rock;
                    });
            }
            if line[0].y == line[1].y {
                (min(line[0].x, line[1].x)..=max(line[0].x, line[1].x))
                    .map(|x| ((x - min_x) as usize, (line[0].y - min_y) as usize))
                    .for_each(|(x, y)| {
                        grid[y][x] = Rock;
                    });
            }
        })
    });

    Cave {
        grid,
        start: Pos {
            x: 500 - min_x,
            y: 0 - min_y,
        },
    }
}

fn build_map_infinite(rocklines: &[Line]) -> Cave {
    let mut map = build_map(rocklines);
    let new_width = (map.grid.len() + 2) * 2;
    let shift = (new_width - map.grid[0].len()) / 2 - 1;
    map.grid.iter_mut().for_each(|l| {
        l.resize(new_width, Empty);
        l.rotate_right(shift);
    });
    map.start.x += shift as i16;
    // Add end lines
    map.grid.push(vec![Empty; new_width]);
    map.grid.push(vec![Rock; new_width]);
    map
}

#[test]
fn test() {
    let rocklines = parse(include_str!("../sample.txt"));
    //part 1
    let cave = build_map(&rocklines);
    println!("{}", cave);
    assert_eq!(
        format!("{}", cave),
        "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.
",
    );
    let res = max_caught_sand(&rocklines);
    assert_eq!(res, 24);
    //part 2
    let res = max_caught_sand_inf(&rocklines);
    assert_eq!(res, 93);
}
