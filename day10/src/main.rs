use crate::Ins::*;

fn main() {
    let instructions = parse(include_str!("../input.txt"));
    //part 1
    let res = signal_strength(&instructions);
    println!("Signal strength: {}", res);
    //part 2
    let res = crt(&instructions);
    println!("CRT:\n{}", res);
}
fn parse(input: &str) -> Vec<Ins> {
    input
        .lines()
        .map(|x| match &x[..4] {
            "noop" => Noop,
            "addx" => Addx(x[5..].parse().expect("not int")),
            _ => panic!("not instruction"),
        })
        .collect()
}

#[derive(Debug)]
enum Ins {
    Noop,
    Addx(i8),
}
fn signal_strength(instructions: &[Ins]) -> i32 {
    let mut s = State::default();
    instructions
        .iter()
        .flat_map(|_| {
            let pc_start = s.pc;
            let mut res = Vec::new();
            while s.pc == pc_start {
                run_cycle(instructions, &mut s);
                /*
                println!(
                    "pc: {:02} {:?} -> {:03} (waiting: {:02})",
                    s.pc,
                    instructions[s.pc % instructions.len()],
                    s.x,
                    s.wait_add,
                );
                */
                res.push(s.x)
            }
            res
        })
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .filter(|(i, _)| *i >= 20 && ((*i - 20) % 40) == 0)
        /*
        .filter(|(i, x)| {
            println!("{i}: {x}");
            true
        })
        */
        .map(|(i, x)| (i as i32) * x)
        .sum()
}

#[derive(Debug)]
struct State {
    x: i32,
    pc: usize,
    wait: u8,
    wait_add: i8,
}
impl Default for State {
    fn default() -> Self {
        State {
            x: 1,
            pc: 0,
            wait: 0,
            wait_add: 0,
        }
    }
}

fn run_cycle(instructions: &[Ins], s: &mut State) {
    if s.wait_add != 0 {
        s.x += s.wait_add as i32;
        s.wait_add = 0;
    }
    if s.pc >= instructions.len() {
        return;
    }
    match instructions[s.pc] {
        Noop => {
            s.pc += 1;
        }
        Addx(i) => {
            if s.wait == 0 {
                s.wait = 1;
                return;
            }
            if s.wait > 0 {
                s.wait -= 1;
                if s.wait == 0 {
                    // now we can run
                    s.wait_add = i;
                    s.pc += 1;
                }
            }
        }
    }
}

fn crt(instructions: &[Ins]) -> String {
    let mut s = State::default();
    instructions
        .iter()
        .flat_map(|_| {
            let pc_start = s.pc;
            let mut res = Vec::new();
            while s.pc == pc_start {
                run_cycle(instructions, &mut s);
                /*
                println!(
                    "pc: {:02} {:?} -> {:03} (waiting: {:02})",
                    s.pc,
                    instructions[s.pc % instructions.len()],
                    s.x,
                    s.wait_add,
                );
                */
                res.push(s.x)
            }
            res
        })
        .enumerate()
        //.map(|(i, x)| (i + 1, x))
        .map(|(i, x)| (i, (x - 1..=x + 1).contains(&(i as i32 % 40))))
        .map(|(i, val)| (i, if val { '#' } else { '.' }))
        .flat_map(|(i, val)| {
            if i > 1 && i % 40 == 0 {
                vec!['\n', val]
            } else {
                vec![val]
            }
        })
        .collect()
}

#[test]
fn test() {
    let instructions = parse(
        "noop
addx 3
addx -5",
    );
    let mut s = State::default();
    run_cycle(&instructions, &mut s);
    assert_eq!(s.x, 1);
    run_cycle(&instructions, &mut s);
    assert_eq!(s.x, 1);
    run_cycle(&instructions, &mut s);
    assert_eq!(s.x, 1);
    run_cycle(&instructions, &mut s);
    assert_eq!(s.x, 4);
    run_cycle(&instructions, &mut s);
    assert_eq!(s.x, 4);
    run_cycle(&instructions, &mut s);
    assert_eq!(s.x, -1);
    let instructions = parse(include_str!("../sample.txt"));
    //part 1
    let res = signal_strength(&instructions);
    assert_eq!(res, 13140);
    //part 2
    let res = crt(&instructions);
    assert_eq!(
        res,
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );
}
