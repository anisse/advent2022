use advent2022::*;

//use itertools::Itertools;

use crate::Move::*;
use crate::Tile::*;

fn main() {
    let (map, pass) = parse(input!());
    //part 1
    let res = walk_map(&map, &pass);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&map, &pass);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> (Map, Password) {
    let mut inp = input.split("\n\n");
    let mapi = inp.next().expect("map");
    let passi = inp
        .next()
        .expect("password")
        .lines()
        .next()
        .expect("password line")
        .chars();
    let map: Map = mapi
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    ' ' => OffMap,
                    '.' => Open,
                    '#' => Wall,
                    _ => panic!("unknown char {c}"),
                })
                .collect()
        })
        .collect();
    let mut pass: Password = vec![];
    let mut ns = String::new();
    for c in passi {
        if c.is_ascii_digit() {
            ns.push(c);
            continue;
        }
        let n: u8 = ns.parse().expect("an int");
        ns.clear();
        pass.push(Forward(n));
        pass.push(match c {
            'L' => Left,
            'R' => Right,
            _ => panic!("unknown dir {c}"),
        })
    }
    let n: u8 = ns.parse().expect("an int");
    ns.clear();
    pass.push(Forward(n));

    (map, pass)
}

type Map = Vec<Vec<Tile>>;
type MapSlice = [Vec<Tile>];

type Password = Vec<Move>;
type PasswordSlice = [Move];

#[derive(Debug)]
enum Move {
    Forward(u8),
    Left,
    Right,
}
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}
impl From<isize> for Facing {
    fn from(value: isize) -> Self {
        match value % 4 {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => unreachable!(),
        }
    }
}

impl Facing {
    fn rotate(&mut self, m: &Move) {
        let rot = match m {
            Left => -1,
            Right => 1,
            _ => 0,
        };
        *self = Self::from(*self as isize + rot + 4);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    OffMap,
    Open,
    Wall,
}

#[derive(Debug, Clone)]
struct Pos {
    x: isize,
    y: isize,
}
impl Pos {
    fn single_move(&mut self, facing: Facing, m: &MapSlice) -> bool {
        let start_pos = self.clone();
        match facing {
            Facing::Right => self.move_wrap_right(m),
            Facing::Down => self.move_wrap_down(m),
            Facing::Left => self.move_wrap_left(m),
            Facing::Up => self.move_wrap_up(m),
        }
        println!("Moving {facing:?}, {start_pos:?} -> {self:?}");
        if m[self.y as usize][self.x as usize] == Wall {
            self.x = start_pos.x;
            self.y = start_pos.y;
            return false;
        }
        true
    }
    fn move_wrap_right(&mut self, m: &MapSlice) {
        self.x += 1;
        if self.x as usize >= m[self.y as usize].len()
            || m[self.y as usize][self.x as usize] == OffMap
        {
            for x in (0..(self.x - 1)).rev() {
                if m[self.y as usize][x as usize] == OffMap {
                    self.x = x + 1;
                    return;
                }
            }
            self.x = 0;
        }
    }
    fn move_wrap_down(&mut self, m: &MapSlice) {
        self.y += 1;
        if self.y as usize >= m.len()
            || self.x >= m[self.y as usize].len() as isize
            || m[self.y as usize][self.x as usize] == OffMap
        {
            for y in (0..(self.y - 1)).rev() {
                println!(
                    "evaluating {}x{y}: {:?}",
                    self.x, m[y as usize][self.x as usize]
                );
                if self.x >= m[y as usize].len() as isize
                    || m[y as usize][self.x as usize] == OffMap
                {
                    self.y = y + 1;
                    return;
                }
            }
            self.y = 0;
        }
    }
    fn move_wrap_left(&mut self, m: &MapSlice) {
        self.x -= 1;
        if self.x < 0 || m[self.y as usize][self.x as usize] == OffMap {
            for x in (self.x + 1)..(m[self.y as usize].len() as isize) {
                if m[self.y as usize][x as usize] == OffMap {
                    self.x = x - 1;
                    return;
                }
            }
            self.x = m[self.y as usize].len() as isize - 1;
        }
    }
    fn move_wrap_up(&mut self, m: &MapSlice) {
        self.y -= 1;
        if self.y < 0 || m[self.y as usize][self.x as usize] == OffMap {
            for y in (self.y + 1)..(m.len() as isize) {
                if self.x >= m[y as usize].len() as isize
                    || m[y as usize][self.x as usize] == OffMap
                {
                    self.y = y - 1;
                    return;
                }
            }
            self.y = m.len() as isize - 1;
        }
    }
}

fn walk_map(m: &MapSlice, p: &PasswordSlice) -> usize {
    let mut facing = Facing::Right;
    let mut pos = Pos {
        x: m[0]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == Open)
            .expect("a start pos")
            .0 as isize,
        y: 0,
    };
    p.iter()
        .for_each(|c| single_move(m, c, &mut pos, &mut facing));
    println!("Done, now at pos {pos:?} facing {facing:?}");
    1000 * (pos.y + 1) as usize + 4 * (pos.x + 1) as usize + facing as usize
}

fn single_move(map: &MapSlice, mov: &Move, pos: &mut Pos, facing: &mut Facing) {
    match mov {
        Forward(mut n) => {
            while n > 0 && pos.single_move(*facing, map) {
                n -= 1;
            }
        }
        Left | Right => facing.rotate(mov),
    }
}

#[test]
fn test() {
    let (map, pass) = parse(sample!());
    //part 1
    let res = walk_map(&map, &pass);
    assert_eq!(res, 6032);
    //part 2
    // let res = operation2(&map, &pass);
    // assert_eq!(res, 42);
}
