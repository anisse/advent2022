use advent2022::*;

use std::collections::VecDeque;

use crate::Direction::*;
use crate::Tile::*;

fn main() {
    let mut map = parse(input!());
    //part 1
    let res = spread_rounds(&mut map, 10);
    println!("Empty tiles after 10 rounds: {}", res);
    //part 2
    let mut map = parse(input!());
    let res = first_stable_round(&mut map);
    println!("First stable round: {}", res);
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
impl Pos {
    fn adjacent(&self) -> Vec<Pos> {
        vec![
            Pos {
                x: self.x - 1,
                y: self.y - 1,
            },
            Pos {
                x: self.x,
                y: self.y - 1,
            },
            Pos {
                x: self.x + 1,
                y: self.y - 1,
            },
            Pos {
                x: self.x - 1,
                y: self.y + 1,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
            Pos {
                x: self.x + 1,
                y: self.y + 1,
            },
            Pos {
                x: self.x - 1,
                y: self.y,
            },
            Pos {
                x: self.x + 1,
                y: self.y,
            },
        ]
    }
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
        /*
        println!("before round {r}");
        print_map(map);
        */
        spread_single_round(map, &mut origin, Direction::from(r));
    }
    /*
    println!("final");
    print_map(map);
    */
    // then re-shrink map for minimal size
    shrink_map(map, &mut origin);
    map.iter().flatten().filter(|t| **t == Empty).count()
}
fn first_stable_round(map: &mut Map) -> usize {
    let mut origin = Pos { x: 0, y: 0 };
    let mut r = 0;
    while spread_single_round(map, &mut origin, Direction::from(r)) {
        r += 1;
    }

    r + 1
}

fn spread_single_round(map: &mut Map, origin: &mut Pos, dir: Direction) -> bool {
    // First spread tentatively
    let elf_real_pos: Vec<Pos> = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, t)| **t == Elf)
                .map(move |(x, _)| (x, y))
        })
        .map(|(x, y)| Pos {
            x: x as i32 - origin.x,
            y: y as i32 - origin.y,
        })
        .collect();

    elf_real_pos.iter().for_each(|e| {
        move_elf(
            map,
            origin,
            Pos {
                x: e.x + origin.x,
                y: e.y + origin.y,
            },
            dir,
        );
    });
    /*
    println!("Before resolution {dir:?}");
    print_map(map);
    */
    let mut moved = false;
    // then do the resolution
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let MovingElves { prev } = &map[y][x] {
                let prev = prev.clone();
                match prev.len().cmp(&1) {
                    std::cmp::Ordering::Less => unreachable!(),
                    std::cmp::Ordering::Equal => {
                        // success ! this is now an elf
                        let map_pos = Pos {
                            x: prev[0].x + origin.x,
                            y: prev[0].y + origin.y,
                        };
                        map[map_pos.y as usize][map_pos.x as usize] = Empty;
                        map[y][x] = Elf;
                        moved = true;
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

    moved
}
fn move_elf(map: &mut Map, origin: &mut Pos, map_pos: Pos, first_dir: Direction) {
    if map_pos
        .adjacent()
        .iter()
        .all(|next| needs_grow(map, next) || map[next.y as usize][next.x as usize] != Elf)
    {
        //No elf around, no need to move
        return;
    }
    let real_pos = Pos {
        x: map_pos.x - origin.x,
        y: map_pos.y - origin.y,
    };
    for d in 0..4 {
        let dir = Direction::from(d + first_dir as usize);
        if dir
            .iter_pos(&map_pos)
            .all(|next| needs_grow(map, &next) || map[next.y as usize][next.x as usize] != Elf)
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

fn print_map(map: &Map) {
    map.iter().for_each(|l| {
        l.iter().for_each(|c| match c {
            Empty => print!("."),
            Elf => print!("#"),
            MovingElves { prev } => print!("{}", prev.len()),
        });
        println!();
    });
}

#[test]
fn test() {
    let mut smol_map = parse(
        ".....\n\
        ..##.\n\
        ..#..\n\
        .....\n\
        ..##.\n\
        .....",
    );
    spread_rounds(&mut smol_map, 3);
    assert_eq!(
        smol_map,
        parse(
            "..#..\n\
            ....#\n\
            #....\n\
            ....#\n\
            .....\n\
            ..#.."
        )
    );

    let mut map = parse(sample!());
    //part 1
    let res = spread_rounds(&mut map, 10);
    assert_eq!(res, 110);
    //part 2
    let mut map = parse(sample!());
    let res = first_stable_round(&mut map);
    assert_eq!(res, 20);
}
