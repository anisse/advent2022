use advent2022::*;
use std::collections::BinaryHeap;

fn main() {
    let map = parse(input!());
    //part 1
    let res = shortest_path(&map);
    println!("Shortest path from start: {}", res);
    //part 2
    let res = shortest_path2(&map);
    println!("Shortest path from any a: {}", res);
}
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|x| x.chars().map(|c| c as u8).collect())
        .collect()
}
fn shortest_path(map: &[Vec<u8>]) -> usize {
    let mut map2 = map.to_vec();
    let (end_x, end_y) = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, c)| (x, y, *c as u8)))
        .find(|(_, _, c)| *c == b'E')
        .map(|(x, y, _)| (x, y))
        .expect("no end");
    let (start_x, start_y) = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, c)| (x, y, *c as u8)))
        .find(|(_, _, c)| *c == b'S')
        .map(|(x, y, _)| (x, y))
        .expect("no start");
    map2[start_y][start_x] = b'a';
    map2[end_y][end_x] = b'z';
    shortest_path_iter(&map2, start_x, start_y, end_x, end_y)
}
fn shortest_path2(map: &[Vec<u8>]) -> usize {
    let mut map2 = map.to_vec();
    let (end_x, end_y) = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, c)| (x, y, *c as u8)))
        .find(|(_, _, c)| *c == b'E')
        .map(|(x, y, _)| (x, y))
        .expect("no end");
    map2[end_y][end_x] = b'z';
    map.iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, c)| (x, y, *c as u8)))
        .filter(|(_, _, c)| *c == b'a')
        .map(|(x, y, _)| shortest_path_iter(&map2, x, y, end_x, end_y))
        .min()
        .expect("no min")
}
#[derive(Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
    total: usize,
}
impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.total.cmp(&self.total)
    }
}
impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path_iter(
    map: &[Vec<u8>],
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) -> usize {
    let mut next: BinaryHeap<Pos> = BinaryHeap::new();
    let mut minmap = vec![vec![usize::MAX; map[0].len()]; map.len()];
    let map_end_x = map[0].len() - 1;
    let map_end_y = map.len() - 1;

    next.push(Pos {
        x: start_x,
        y: start_y,
        total: 0,
    });
    while let Some(Pos { x, y, total }) = next.pop() {
        //println!("Exploring {x} {y} : \"{} path={total}", map[y][x] as char);
        if total >= minmap[y][x] {
            continue;
        }
        if x == end_x && y == end_y {
            return total;
        }
        minmap[y][x] = total;
        for i in 0..4 {
            let (x2, y2) = match i {
                0 if x < map_end_x => (x + 1, y),
                1 if y < map_end_y => (x, y + 1),
                2 if x > 0 => (x - 1, y),
                3 if y > 0 => (x, y - 1),
                _ => continue,
            };
            /*
            println!(
                "Evaluating next {x2} {y2} : '{}', diff={}",
                map[y2][x2] as char,
                map[y2][x2] as i16 - map[y][x] as i16
            );
            */
            if (map[y2][x2] as i16 - map[y][x] as i16) > 1 {
                continue;
            }
            //println!("OK");
            next.push(Pos {
                x: x2,
                y: y2,
                total: total + 1,
            });
        }
    }
    usize::MAX
}

#[test]
fn test() {
    let map = parse(sample!());
    //part 1
    let res = shortest_path(&map);
    assert_eq!(res, 31);
    //part 2
    let res = shortest_path2(&map);
    assert_eq!(res, 29);
}
