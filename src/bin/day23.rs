use advent2022::*;

use std::collections::VecDeque;

use crate::Direction::*;
use crate::Tile::*;

fn main() {
    let mut map = parse(input!());
    //part 1
    let res = spread_rounds(&mut map, 10);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&map);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Empty,
                    '#' => Elf,
                    _ => panic!("unknown char {c}"),
                })
                .collect()
        })
        .collect()
}

type Map = VecDeque<VecDeque<Tile>>;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Empty,
    Elf,
    MovingElves { prev: Vec<Pos> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Pos {
    x: i32,
    y: i32,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}
impl Direction {
    fn next(self, start: &Pos) -> Pos {
        match self {
            North => Pos {
                x: start.x,
                y: start.y - 1,
            },
            South => Pos {
                x: start.x,
                y: start.y + 1,
            },
            West => Pos {
                x: start.x - 1,
                y: start.y,
            },
            East => Pos {
                x: start.x + 1,
                y: start.y,
            },
        }
    }
    fn iter_pos(self, start: &Pos) -> PosIterator {
        PosIterator {
            vec: match self {
                North => vec![
                    Pos {
                        x: start.x - 1,
                        y: start.y - 1,
                    },
                    Pos {
                        x: start.x,
                        y: start.y - 1,
                    },
                    Pos {
                        x: start.x + 1,
                        y: start.y - 1,
                    },
                ],
                South => vec![
                    Pos {
                        x: start.x - 1,
                        y: start.y + 1,
                    },
                    Pos {
                        x: start.x,
                        y: start.y + 1,
                    },
                    Pos {
                        x: start.x + 1,
                        y: start.y + 1,
                    },
                ],
                West => vec![
                    Pos {
                        x: start.x - 1,
                        y: start.y - 1,
                    },
                    Pos {
                        x: start.x - 1,
                        y: start.y,
                    },
                    Pos {
                        x: start.x - 1,
                        y: start.y + 1,
                    },
                ],
                East => vec![
                    Pos {
                        x: start.x + 1,
                        y: start.y - 1,
                    },
                    Pos {
                        x: start.x + 1,
                        y: start.y,
                    },
                    Pos {
                        x: start.x + 1,
                        y: start.y + 1,
                    },
                ],
            },
        }
    }
}

struct PosIterator {
    vec: Vec<Pos>,
}
impl Iterator for PosIterator {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        self.vec.pop()
    }
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value % 4 {
            0 => North,
            1 => South,
            2 => West,
            3 => East,
            _ => unreachable!(),
        }
    }
}

fn spread_rounds(map: &mut Map, rounds: usize) -> usize {
    let mut origin = Pos { x: 0, y: 0 };
    for r in 0..rounds {
        spread_single_round(map, &mut origin, Direction::from(r));
    }
    map.iter().flatten().filter(|t| **t == Empty).count()
}

fn spread_single_round(map: &mut Map, origin: &mut Pos, dir: Direction) {
    // First spread tentatively
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != Elf {
                continue;
            }
            move_elf(
                map,
                origin,
                Pos {
                    y: y as i32,
                    x: x as i32,
                },
                dir,
            );
        }
    }
    // then do the resolution
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let MovingElves { prev } = &map[y][x] {
                let prev = prev.clone();
                match prev.len().cmp(&1) {
                    std::cmp::Ordering::Less => unreachable!(),
                    std::cmp::Ordering::Equal => {
                        // success ! this is now an elf
                        map[y][x] = Elf
                    }
                    std::cmp::Ordering::Greater => {
                        // No dice, let's move all elves back to their origin position
                        for elf in prev.iter() {
                            let map_pos = Pos {
                                x: elf.x + origin.x,
                                y: elf.y + origin.y,
                            };
                            map[map_pos.y as usize][map_pos.x as usize] = Elf;
                        }
                        map[y][x] = Empty;
                    }
                }
            }
        }
    }

    // then re-shrink map for minimal size
    shrink_map(map, origin);
}
fn move_elf(map: &mut Map, origin: &mut Pos, map_pos: Pos, first_dir: Direction) {
    let real_pos = Pos {
        x: map_pos.x - origin.x,
        y: map_pos.y - origin.y,
    };
    for d in 0..4 {
        let dir = Direction::from(d + first_dir as usize);
        if dir
            .iter_pos(&map_pos)
            .all(|next| needs_grow(map, &next) || map[next.y as usize][next.x as usize] == Empty)
        {
            let mut new_map_pos = dir.next(&map_pos);
            let new_real_pos = Pos {
                x: new_map_pos.x - origin.x,
                y: new_map_pos.y - origin.y,
            };
            if needs_grow(map, &new_map_pos) {
                grow_map(map, origin, dir);
                new_map_pos = Pos {
                    x: new_real_pos.x + origin.x,
                    y: new_real_pos.y + origin.y,
                }
            }
            let next_tile = &mut map[new_map_pos.y as usize][new_map_pos.x as usize];
            if let MovingElves { prev } = next_tile {
                prev.push(real_pos)
            } else {
                *next_tile = MovingElves {
                    prev: vec![real_pos],
                }
            }
            map[map_pos.y as usize][map_pos.x as usize] = Empty;
            return;
        }
    }
}
fn needs_grow(map: &Map, next: &Pos) -> bool {
    next.y < 0 || next.x < 0 || next.y >= map.len() as i32 || next.x >= map[0].len() as i32
}

fn grow_map(map: &mut Map, origin: &mut Pos, dir: Direction) {
    match dir {
        North => {
            map.push_front(VecDeque::from(vec![Empty; map[0].len()]));
            origin.y += 1;
        }
        South => {
            map.push_back(VecDeque::from(vec![Empty; map[0].len()]));
        }
        West => {
            (0..map.len()).for_each(|y| map[y].push_front(Empty));
            origin.x += 1;
        }
        East => {
            (0..map.len()).for_each(|y| map[y].push_back(Empty));
        }
    }
}
fn shrink_map(map: &mut Map, origin: &mut Pos) {
    //check each four border; if empty, remove
    if map[0].iter().all(|t| *t == Empty) {
        // shrink first row
        map.pop_front();
        origin.x -= 1
    }
    if map[map.len() - 1].iter().all(|t| *t == Empty) {
        // shrink last row
        map.pop_back();
    }
    if (0..map.len())
        .filter(|y| map[*y][map[0].len() - 1] != Empty)
        .count()
        == 0
    {
        // shrink first col
        (0..map.len()).for_each(|y| {
            map[y].pop_front();
        });
        origin.y -= 1;
    }
    if (0..map.len())
        .filter(|y| map[*y][map[0].len() - 1] != Empty)
        .count()
        == 0
    {
        // shrink last col
        (0..map.len()).for_each(|y| {
            map[y].pop_back();
        });
    }
}

#[test]
fn test() {
    let mut map = parse(sample!());
    //part 1
    let res = spread_rounds(&mut map, 10);
    assert_eq!(res, 110);
    //part 2
    // let res = operation2(&map);
    // assert_eq!(res, 42);
}
