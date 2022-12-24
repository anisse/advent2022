use advent2022::*;

use crate::Tile::*;

fn main() {
    let map = parse(input!());
    //part 1
    let res = fastest_path(&map);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&map);
    //println!("Summary2: {}", res);
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
    let mut next_pos = vec![Pos { x: 0, y: 0 }];
    let mut new_next = vec![];
    let mut step = 0;
    let map_max = Pos {
        x: map[0].len() as u8,
        y: map.len() as u8,
    };
    let target_pos = Pos {
        x: map[0].len() as u8 - 1,
        y: map.len() as u8 - 1,
    };
    'outer: loop {
        step += 1;
        while let Some(nextp) = next_pos.pop() {
            //println!("At step {step} evaluating {nextp:?}");
            if has_blizzard(map, &nextp, step) {
                //   println!("Is estimated to have blizzard");
                continue;
            }
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
    dbg!(&map);
    //part 1
    let res = fastest_path(&map);
    assert_eq!(res, 18);
    //part 2
    // let res = operation2(&map);
    // assert_eq!(res, 42);
}
