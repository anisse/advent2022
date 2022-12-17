use crate::Jet::*;
use crate::Tile::*;
use advent2022::*;

fn main() {
    let jets = parse(input!());
    //part 1
    let res = simulate(&jets, 2022);
    println!("Summary: {}", res);
    //part 2
    let res = simulate_big(&jets, 1000000000000);
    println!("Summary2: {}", res);
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Stone,
    Space,
}
type Rock = Vec<Vec<Tile>>;

fn p(s: &str) -> Vec<Tile> {
    s.chars()
        .map(|c| match c {
            '#' => Stone,
            _ => Space,
        })
        .collect()
}
fn rocks() -> Vec<Rock> {
    vec![
        vec![p("####")],
        vec![p(".#."), p("###"), p(".#.")],
        vec![p("..#"), p("..#"), p("###")],
        vec![p("#"), p("#"), p("#"), p("#")],
        vec![p("##"), p("##")],
    ]
}

#[derive(Debug, Clone, Copy)]
enum Jet {
    Left,
    Right,
}
impl Jet {
    fn dir(&self) -> i8 {
        match self {
            Left => -1,
            Right => 1,
        }
    }
}

fn parse(input: &str) -> Vec<Jet> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Right,
            '<' => Left,
            _ => unreachable!(),
        })
        .collect()
}
type Tower = Vec<[Tile; 7]>;
type TowerSlice = [[Tile; 7]];
fn simulate(jets: &[Jet], rounds: usize) -> usize {
    let rocks = rocks();
    let mut tower: Tower = Vec::new();
    let mut j = 0;
    for r in 0..rounds {
        j += add_rock(&mut tower, &rocks[r % rocks.len()], jets, j);
        //println!("After round {r} in {j} steps:");
        //print_tower(&tower);
    }
    tower.len()
}
fn simulate_big(jets: &[Jet], rounds: usize) -> usize {
    let rocks = rocks();
    let mut tower: Tower = Vec::new();
    let mut j = 0;
    // Attempt to find the "loop" (hoping it exists), this way we know its len, we know how it will
    // repeat, modulo the "big" rounds, simulate the rest, multiply, add and we have the result.
    //let mut first = true;
    let mut prev_height = 0;
    let mut prev_tower_height = 0;
    //let mut tower_top_match = [Space, Space, Space, Space, Space, Space, Space];
    let mut r = 0;
    let mut prev_rocks = 0;
    let mut loop_rocks;
    loop {
        for r in 0..rocks.len() {
            j += add_rock(&mut tower, &rocks[r % rocks.len()], jets, j);
        }
        r += rocks.len();
        // magic value found after eyeballing the output: it should be
        // the cycle start
        if j % jets.len() == 28 {
            // LOOP !!!
            // now measure, and hope it repeats at next iteration
            let height = tower.len() - prev_tower_height;
            loop_rocks = r - prev_rocks;
            println!(
                "loop found at {j} tower is {}, height = {height} {:?} tower was {prev_tower_height}",
                tower.len(),
                height.cmp(&prev_height)
            );
            if prev_height != height {
                // no dice, continue
                prev_height = height;
                prev_tower_height = tower.len();
                prev_rocks = r;
                continue;
            }
            break;
        }
    }
    for r in 0..((rounds - r) % loop_rocks) {
        j += add_rock(&mut tower, &rocks[r % rocks.len()], jets, j);
    }
    let remaining_height = tower.len() - prev_tower_height - prev_height;

    prev_tower_height + prev_height * ((rounds - r) / loop_rocks + 1) + remaining_height
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stone => write!(f, "#"),
            Space => write!(f, " "),
        }
    }
}
fn print_tower(t: &TowerSlice) {
    t.iter().rev().for_each(|l| {
        l.iter().for_each(|c| print!("{}", c));
        println!();
    });
}

fn add_rock(tower: &mut Tower, rock: &Rock, jets: &[Jet], j_start: usize) -> usize {
    let mut rock_y = tower.len() + 3;
    for _ in 0..(3 + rock.len() + 1) {
        tower.push([Space, Space, Space, Space, Space, Space, Space]);
    }
    let mut step = 0;
    let mut rock_x = 2;
    loop {
        let jet = jets[(j_start + step) % jets.len()];
        if jet_push(jet, rock, rock_x, rock_y, tower) {
            //println!("Push to {jet:?} OK");
            rock_x += jet.dir();
        }
        step += 1;
        if rock_fall(rock, rock_x, rock_y, tower) {
            break;
        }
        rock_y -= 1;
    }
    // Put rock at coordinates
    for (y, l) in rock.iter().rev().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == Stone {
                tower[rock_y + y][rock_x as usize + x] = Stone
            }
        }
    }
    // trim any un-neccessary space
    while tower[tower.len() - 1].iter().all(|c| *c == Space) {
        tower.pop();
    }
    step
}

fn rock_width(r: &Rock) -> i8 {
    r.iter()
        .map(|l| l.iter().filter(|c| **c == Stone).count())
        .max()
        .expect("width") as i8
}

fn jet_push(jet: Jet, rock: &Rock, rock_x: i8, rock_y: usize, tower: &TowerSlice) -> bool {
    let width = rock_width(rock) - 1;
    let new_x = rock_x + jet.dir();
    if new_x < 0 || (new_x + width) >= tower[0].len() as i8 {
        //println!("Push to {jet:?} failed because of new_x:={new_x} + width = {width}");
        return false;
    }
    // Now check overlaps with actual tower content
    !rock_overlap(rock, new_x as usize, rock_y, tower)
}

fn rock_overlap(rock: &Rock, rock_x: usize, rock_y: usize, tower: &TowerSlice) -> bool {
    //println!("checking overlap rock at {rock_x}x{rock_y}: {rock:?}");
    for (y, l) in rock.iter().rev().enumerate() {
        for (x, c) in l.iter().enumerate() {
            /*
            println!(
                "Checking at {c:?} at  {}x{} =  {rock_x}+{x}x{rock_y}-{y}",
                rock_x + x,
                rock_y + y
            );
            */
            if *c == Stone && tower[rock_y + y][rock_x + x] == Stone {
                return true;
            }
        }
    }
    false
}

fn rock_fall(rock: &Rock, rock_x: i8, rock_y: usize, tower: &TowerSlice) -> bool {
    /*
    println!(
        "checking fall of rock at {rock_y} -> {} (bottom: {} -> {})",
        rock_y as i32 - 1,
        rock_y as i32 - 1 - rock.len() as i32,
        rock_y as i32 - rock.len() as i32,
    );
    */
    if rock_y.checked_sub(1).is_none() {
        return true;
    }
    rock_overlap(rock, rock_x as usize, rock_y - 1, tower)
}

#[test]
fn test() {
    let jets = parse(sample!());
    //part 1
    let res = simulate(&jets, 2022);
    assert_eq!(res, 3068);
    //part 2
    let res = simulate_big(&jets, 1000000000000);
    assert_eq!(res, 1514285714288);
}
