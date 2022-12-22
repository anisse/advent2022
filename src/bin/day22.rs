use advent2022::*;

//use itertools::Itertools;

use crate::Move::*;
use crate::Tile::*;

fn main() {
    let (map, pass) = parse(input!());
    //part 1
    let res = walk_map(&map, &pass);
    println!("Password on flat edges: {}", res);
    //part 2
    let res = walk_cube(&map, &pass);
    println!("Password on a cube: {}", res);
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
        //println!("Moving {facing:?}, {start_pos:?} -> {self:?}");
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
                /*
                println!(
                    "evaluating {}x{y}: {:?}",
                    self.x, m[y as usize][self.x as usize]
                );
                */
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
    fn to_cube_face(&self, c: &Cube) -> (Self, CubeFace) {
        for face in 0..6 {
            if self.x >= c.map[face].start.x
                && self.x < c.map[face].start.x + c.side
                && self.y >= c.map[face].start.y
                && self.y < c.map[face].start.y + c.side
            {
                // This is the face
                return (
                    Self {
                        x: self.x - c.map[face].start.x,
                        y: self.y - c.map[face].start.y,
                    },
                    CubeFace::from(face),
                );
            }
        }
        panic!("Unknown coord {self:?} on cube {c:?}");
    }
    fn to_map_coord(&self, c: &Cube, face: CubeFace) -> Self {
        Self {
            x: self.x + c.map[face as usize].start.x,
            y: self.y + c.map[face as usize].start.y,
        }
    }
    fn single_move_cube(&mut self, facing: &mut Facing, m: &MapSlice, c: &Cube) -> bool {
        let start_pos = self.clone();
        let start_facing = *facing;
        let (mut cube_pos, mut face) = self.to_cube_face(c);
        match facing {
            Facing::Right => cube_pos.x += 1,
            Facing::Down => cube_pos.y += 1,
            Facing::Left => cube_pos.x -= 1,
            Facing::Up => cube_pos.y -= 1,
        }
        if cube_pos.x < 0 || cube_pos.x >= c.side || cube_pos.y < 0 || cube_pos.y >= c.side {
            let next_face = &c.map[face as usize].next[*facing as usize];
            let end = c.side - 1;
            match (*facing, next_face.facing) {
                (Facing::Right, Facing::Right)
                | (Facing::Down, Facing::Down)
                | (Facing::Left, Facing::Left)
                | (Facing::Up, Facing::Up) => {}

                (Facing::Right, Facing::Left) | (Facing::Left, Facing::Right) => {
                    cube_pos.y = end - cube_pos.y
                }
                (Facing::Down, Facing::Up) | (Facing::Up, Facing::Down) => {
                    cube_pos.x = end - cube_pos.x
                }

                (Facing::Down, Facing::Left) | (Facing::Up, Facing::Right) => {
                    cube_pos.y = cube_pos.x
                }
                (Facing::Right, Facing::Up) | (Facing::Left, Facing::Down) => {
                    cube_pos.x = cube_pos.y
                }

                (Facing::Right, Facing::Down) | (Facing::Left, Facing::Up) => {
                    cube_pos.x = end - cube_pos.y
                }
                (Facing::Down, Facing::Right) | (Facing::Up, Facing::Left) => {
                    cube_pos.y = end - cube_pos.x
                }
            }
            // Common things
            match next_face.facing {
                Facing::Right => cube_pos.x = 0,
                Facing::Down => cube_pos.y = 0,
                Facing::Left => cube_pos.x = end,
                Facing::Up => cube_pos.y = end,
            }
            println!(
                "Changed face from {face:?} ({:?}) to {:?} ({cube_pos:?}), and facing from {facing:?} to {:?}",
                self.to_cube_face(c).0,
                next_face.face, next_face.facing
            );
            assert!(cube_pos.x >= 0);
            assert!(cube_pos.y >= 0);
            assert!(cube_pos.x < c.side);
            assert!(cube_pos.y < c.side);
            *facing = next_face.facing;
            face = next_face.face;
        }
        *self = cube_pos.to_map_coord(c, face);
        if m[self.y as usize][self.x as usize] == Wall {
            println!("Cancelling Wall move {facing:?}, {start_pos:?} -> {self:?}");
            self.x = start_pos.x;
            self.y = start_pos.y;
            *facing = start_facing;
            return false;
        }
        println!("Cube moving {facing:?}, {start_pos:?} -> {self:?}");
        true
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
    //println!("Done, now at pos {pos:?} facing {facing:?}");
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

/*
#[derive(Debug, Clone)]
struct PosCube {
    x: isize,
    y: isize,
    face: CubeFace,
}
*/

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CubeFace {
    Front,
    Right,
    Down,
    Left,
    Up,
    Back,
}
impl From<usize> for CubeFace {
    fn from(value: usize) -> Self {
        match value % 6 {
            0 => Self::Front,
            1 => Self::Right,
            2 => Self::Down,
            3 => Self::Left,
            4 => Self::Up,
            5 => Self::Back,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct NextFace {
    face: CubeFace,
    facing: Facing,
}

#[derive(Debug, Clone)]
struct CubeFacePos {
    next: [NextFace; 4],
    start: Pos,
}

#[derive(Debug, Clone)]
struct Cube {
    side: isize,
    map: [CubeFacePos; 6],
}

fn walk_cube(m: &MapSlice, p: &PasswordSlice) -> usize {
    let mut facing = Facing::Right;
    let side = m
        .iter()
        .map(|l| l.iter().filter(|c| **c != OffMap).count())
        .min()
        .expect("side length") as isize;
    let cube = match side {
        // hardcode sample and input here
        4 => Cube {
            side,
            map: [
                CubeFacePos {
                    //Front
                    start: Pos { x: 2 * side, y: 0 }, //Front,
                    next: [
                        NextFace {
                            // Facing Right
                            face: CubeFace::Right,
                            facing: Facing::Left,
                        },
                        NextFace {
                            //Facing Down
                            face: CubeFace::Down,
                            facing: Facing::Down,
                        },
                        NextFace {
                            //Facing Left
                            face: CubeFace::Left,
                            facing: Facing::Down,
                        },
                        NextFace {
                            //Facing Up
                            face: CubeFace::Up,
                            facing: Facing::Up,
                        },
                    ],
                },
                CubeFacePos {
                    //Right
                    start: Pos {
                        x: 3 * side,
                        y: 2 * side,
                    }, //Right,
                    next: [
                        NextFace {
                            // Facing Right
                            face: CubeFace::Front,
                            facing: Facing::Left,
                        },
                        NextFace {
                            //Facing Down
                            face: CubeFace::Up,
                            facing: Facing::Right,
                        },
                        NextFace {
                            //Facing Left
                            face: CubeFace::Back,
                            facing: Facing::Left,
                        },
                        NextFace {
                            //Facing Up
                            face: CubeFace::Down,
                            facing: Facing::Left,
                        },
                    ],
                },
                CubeFacePos {
                    //Down
                    start: Pos {
                        x: 2 * side,
                        y: side,
                    }, //Down,
                    next: [
                        NextFace {
                            // Facing Right
                            face: CubeFace::Right,
                            facing: Facing::Down,
                        },
                        NextFace {
                            //Facing Down
                            face: CubeFace::Back,
                            facing: Facing::Down,
                        },
                        NextFace {
                            //Facing Left
                            face: CubeFace::Left,
                            facing: Facing::Left,
                        },
                        NextFace {
                            //Facing Up
                            face: CubeFace::Front,
                            facing: Facing::Up,
                        },
                    ],
                },
                CubeFacePos {
                    //Left
                    start: Pos { x: side, y: side }, //Left,
                    next: [
                        NextFace {
                            // Facing Right
                            face: CubeFace::Down,
                            facing: Facing::Right,
                        },
                        NextFace {
                            //Facing Down
                            face: CubeFace::Back,
                            facing: Facing::Right,
                        },
                        NextFace {
                            //Facing Left
                            face: CubeFace::Up,
                            facing: Facing::Left,
                        },
                        NextFace {
                            //Facing Up
                            face: CubeFace::Front,
                            facing: Facing::Right,
                        },
                    ],
                },
                CubeFacePos {
                    // Up
                    start: Pos { x: 0, y: side }, //Up,
                    next: [
                        NextFace {
                            // Facing Right
                            face: CubeFace::Left,
                            facing: Facing::Right,
                        },
                        NextFace {
                            //Facing Down
                            face: CubeFace::Back,
                            facing: Facing::Up,
                        },
                        NextFace {
                            //Facing Left
                            face: CubeFace::Right,
                            facing: Facing::Up,
                        },
                        NextFace {
                            //Facing Up
                            face: CubeFace::Front,
                            facing: Facing::Down,
                        },
                    ],
                },
                CubeFacePos {
                    // Back
                    start: Pos {
                        x: 2 * side,
                        y: 2 * side,
                    }, //Back,
                    next: [
                        NextFace {
                            // Facing Right
                            face: CubeFace::Right,
                            facing: Facing::Right,
                        },
                        NextFace {
                            //Facing Down
                            face: CubeFace::Up,
                            facing: Facing::Up,
                        },
                        NextFace {
                            //Facing Left
                            face: CubeFace::Left,
                            facing: Facing::Up,
                        },
                        NextFace {
                            //Facing Up
                            face: CubeFace::Down,
                            facing: Facing::Up,
                        },
                    ],
                },
            ],
        },
        50 => Cube {
            side,
            map: [
                CubeFacePos {
                    //Front
                    start: Pos { x: side, y: 0 }, //Front,
                    next: [
                        NextFace {
                            // Facing Right
                            face: CubeFace::Right,
                            facing: Facing::Right,
                        },
                        NextFace {
                            //Facing Down
                            face: CubeFace::Down,
                            facing: Facing::Down,
                        },
                        NextFace {
                            //Facing Left
                            face: CubeFace::Left,
                            facing: Facing::Right,
                        },
                        NextFace {
                            //Facing Up
                            face: CubeFace::Up,
                            facing: Facing::Right,
                        },
                    ],
                },
                CubeFacePos {
                    //Right
                    start: Pos { x: 2 * side, y: 0 }, //Right,
                    next: [
                        NextFace {
                            // Facing Right
                            face: CubeFace::Back,
                            facing: Facing::Left,
                        },
                        NextFace {
                            //Facing Down
                            face: CubeFace::Down,
                            facing: Facing::Left,
                        },
                        NextFace {
                            //Facing Left
                            face: CubeFace::Front,
                            facing: Facing::Left,
                        },
                        NextFace {
                            //Facing Up
                            face: CubeFace::Up,
                            facing: Facing::Up,
                        },
                    ],
                },
                CubeFacePos {
                    //Down
                    start: Pos { x: side, y: side }, //Down,
                    next: [
                        NextFace {
                            // Facing Right
                            face: CubeFace::Right,
                            facing: Facing::Up,
                        },
                        NextFace {
                            //Facing Down
                            face: CubeFace::Back,
                            facing: Facing::Down,
                        },
                        NextFace {
                            //Facing Left
                            face: CubeFace::Left,
                            facing: Facing::Down,
                        },
                        NextFace {
                            //Facing Up
                            face: CubeFace::Front,
                            facing: Facing::Up,
                        },
                    ],
                },
                CubeFacePos {
                    //Left
                    start: Pos { x: 0, y: 2 * side }, //Left,
                    next: [
                        NextFace {
                            // Facing Right
                            face: CubeFace::Back,
                            facing: Facing::Right,
                        },
                        NextFace {
                            //Facing Down
                            face: CubeFace::Up,
                            facing: Facing::Down,
                        },
                        NextFace {
                            //Facing Left
                            face: CubeFace::Front,
                            facing: Facing::Right,
                        },
                        NextFace {
                            //Facing Up
                            face: CubeFace::Down,
                            facing: Facing::Right,
                        },
                    ],
                },
                CubeFacePos {
                    // Up
                    start: Pos { x: 0, y: 3 * side }, //Up,
                    next: [
                        NextFace {
                            // Facing Right
                            face: CubeFace::Back,
                            facing: Facing::Up,
                        },
                        NextFace {
                            //Facing Down
                            face: CubeFace::Right,
                            facing: Facing::Down,
                        },
                        NextFace {
                            //Facing Left
                            face: CubeFace::Front,
                            facing: Facing::Down,
                        },
                        NextFace {
                            //Facing Up
                            face: CubeFace::Left,
                            facing: Facing::Up,
                        },
                    ],
                },
                CubeFacePos {
                    // Back
                    start: Pos {
                        x: side,
                        y: 2 * side,
                    }, //Back,
                    next: [
                        NextFace {
                            // Facing Right
                            face: CubeFace::Right,
                            facing: Facing::Left,
                        },
                        NextFace {
                            //Facing Down
                            face: CubeFace::Up,
                            facing: Facing::Left,
                        },
                        NextFace {
                            //Facing Left
                            face: CubeFace::Left,
                            facing: Facing::Left,
                        },
                        NextFace {
                            //Facing Up
                            face: CubeFace::Down,
                            facing: Facing::Up,
                        },
                    ],
                },
            ],
        },
        _ => unreachable!(),
    };
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
        .for_each(|mov| single_move_cube(m, &cube, mov, &mut pos, &mut facing));
    println!("Done, now at pos {pos:?} facing {facing:?}");
    1000 * (pos.y + 1) as usize + 4 * (pos.x + 1) as usize + facing as usize
}

fn single_move_cube(map: &MapSlice, cube: &Cube, mov: &Move, pos: &mut Pos, facing: &mut Facing) {
    match mov {
        Forward(mut n) => {
            while n > 0 && pos.single_move_cube(facing, map, cube) {
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
    let res = walk_cube(&map, &pass);
    assert_eq!(res, 5031);
}
