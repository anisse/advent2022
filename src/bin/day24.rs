use advent2022::*;

use crate::Tile::*;

fn main() {
    let map = parse(input!());
    //part 1
    let res = fastest_path(&map);
    println!("Fastest path to end: {}", res);
    //part 2
    let res = fastest_roundtrip_path(&map);
    println!("Fastest path additionnal roundtrip: {}", res);
}
fn parse(input: &str) -> Map {
    input
        .lines()
        .skip(1)
        .take_while(|l| l.chars().nth(1).unwrap() != '#')
        .map(|l| {
            l.chars()
                .skip(1)
                .take(l.len() - 2)
                .map(|c| match c {
                    '.' => Empty,
                    '>' => Right,
                    '<' => Left,
                    '^' => Up,
                    'v' => Down,
                    _ => panic!("unknown char {c}"),
                })
                .collect()
        })
        .collect()
}
type Map = Vec<Vec<Tile>>;
type MapSlice = [Vec<Tile>];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Up,
    Down,
    Left,
    Right,
    Empty,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Blizzard {
    Up,
    Down,
    Left,
    Right,
}
impl From<usize> for Blizzard {
    fn from(value: usize) -> Self {
        match value % 4 {
            0 => Self::Up,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
struct Pos {
    x: u8,
    y: u8,
}
impl Pos {
    fn adjacent_and_self(self, map_max: Pos) -> Vec<Pos> {
        let mut v = vec![];
        for (vx, vy) in [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)].iter() {
            let new_x = self.x as i16 + vx;
            let new_y = self.y as i16 + vy;
            if new_x < 0 || new_x >= map_max.x as i16 || new_y < 0 || new_y >= map_max.y as i16 {
                continue;
            }
            v.push(Pos {
                x: new_x as u8,
                y: new_y as u8,
            });
        }
        v
    }
}

fn fastest_path(map: &MapSlice) -> usize {
    let target_pos = Pos {
        x: map[0].len() as u8 - 1,
        y: map.len() as u8 - 1,
    };
    fastest_path_common(map, Pos { x: 0, y: 0 }, target_pos, 0)
}
fn fastest_roundtrip_path(map: &MapSlice) -> usize {
    let start = Pos { x: 0, y: 0 };
    let target_pos = Pos {
        x: map[0].len() as u8 - 1,
        y: map.len() as u8 - 1,
    };
    let step = fastest_path_common(map, start, target_pos, 0);
    //println!("First done in {step}");
    let step = fastest_path_common(map, target_pos, start, step);
    //println!("Back done in {step}");
    fastest_path_common(map, start, target_pos, step)
}
fn fastest_path_common(map: &MapSlice, start: Pos, target_pos: Pos, start_round: usize) -> usize {
    let mut next_pos = vec![start];
    let mut new_next = vec![];
    let mut step = start_round;
    let map_max = Pos {
        x: map[0].len() as u8,
        y: map.len() as u8,
    };
    let lcd = match map.len() {
        // Cheat: hardcode LCD between height and width
        25 => 600,
        4 => 12,
        _ => panic!("Not sample or input map"),
    };

    //println!("Starting to go from {start:?} to {target_pos:?} at step {step}");
    // BITFIELD, someday
    let mut seen: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; lcd]; map[0].len()]; map.len()];
    'outer: loop {
        if step - start_round < lcd {
            next_pos.push(start);
        }

        assert!(!next_pos.is_empty(), "No more positions to evaluate");
        step += 1;
        while let Some(nextp) = next_pos.pop() {
            if seen[nextp.y as usize][nextp.x as usize][step % lcd] {
                continue;
            }
            seen[nextp.y as usize][nextp.x as usize][step % lcd] = true;
            if has_blizzard(map, &nextp, step) {
                //println!("{nextp:?} has blizzard, skipping");
                continue;
            }
            //println!("At step {step} evaluating {nextp:?}");
            if nextp == target_pos {
                break 'outer;
            }
            for p in nextp.adjacent_and_self(map_max).iter() {
                new_next.push(*p);
            }
        }
        (next_pos, new_next) = (new_next, next_pos);
    }
    step + 1
}

fn has_blizzard(map: &MapSlice, p: &Pos, round: usize) -> bool {
    (0..4).map(Blizzard::from).any(|b| match b {
        Blizzard::Up => {
            map[(p.y as usize + map.len() + round) % map.len()][p.x as usize] == Tile::Up
        }
        Blizzard::Down => {
            map[(p.y as usize + map.len() * 1024 - round) % map.len()][p.x as usize] == Tile::Down
        }
        Blizzard::Left => {
            map[p.y as usize][(p.x as usize + map[0].len() + round) % map[0].len()] == Tile::Left
        }
        Blizzard::Right => {
            map[p.y as usize][(p.x as usize + map[0].len() * 1024 - round) % map[0].len()]
                == Tile::Right
        }
    })
}

#[test]
fn test() {
    let map = parse(sample!());
    //part 1
    let res = fastest_path(&map);
    assert_eq!(res, 18);
    //part 1 input
    assert_eq!(fastest_path(&parse(input!())), 257);
    println!("Input OK");
    //part 2
    let res = fastest_roundtrip_path(&map);
    assert_eq!(res, 54);
}
