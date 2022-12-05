fn main() {
    let (crates, instructions) = parse(include_str!("../input.txt"));
    //part 1
    let res = movecrates(&crates, &instructions);
    println!("Summary: {}", res);
    //part 2
    //let res = movecrates2(&(crates, instructions));
    //println!("Summary2: {}", res);
}

#[derive(Clone, Debug)]
struct Move {
    qty: u16,
    src: usize,
    dst: usize,
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let len = (input.lines().next().expect("no line").len() + 1) / 4;
    let mut inputgroups = input.split("\n\n");
    let cratesinput = inputgroups.next().expect("no crate group");
    let movesinput = inputgroups.next().expect("no moves group");
    let height = cratesinput.lines().count() - 1;
    let crateschars: Vec<Vec<char>> = cratesinput.lines().map(|l| l.chars().collect()).collect();
    let mut crates = vec![Vec::with_capacity(height); len];
    (0..len).for_each(|c| {
        for h in (0..height).rev() {
            let el = crateschars[h][1 + 4 * c];
            if el != ' ' {
                crates[c].push(el);
            }
        }
    });
    let moves = movesinput
        .lines()
        .map(|l| {
            let mut comp = l.split_ascii_whitespace();
            let qty = comp.nth(1).expect("No qty").parse().expect("not int");
            let src: usize = comp.nth(1).expect("No src").parse().expect("not int");
            let dst: usize = comp.nth(1).expect("No dst").parse().expect("not int");
            Move {
                qty,
                src: src - 1,
                dst: dst - 1,
            }
        })
        .collect();
    (crates, moves)
}
fn movecrates(crates: &[Vec<char>], instructions: &[Move]) -> String {
    let mut crates = crates.to_owned();
    for m in instructions.iter() {
        (0..m.qty).for_each(|_| {
            let el = crates[m.src].pop().expect("nothing to pop !");
            crates[m.dst].push(el);
        });
    }

    crates
        .iter()
        .map(|c| *c.last().expect("no last char"))
        .collect()
}

#[test]
fn test() {
    let (crates, instructions) = parse(include_str!("../sample.txt"));
    //part 1
    dbg!(&crates);
    dbg!(&instructions);
    let res = movecrates(&crates, &instructions);
    assert_eq!(res, "CMZ");
    //part 2
    // let res = movecrates2(&(crates, instructions));
    // assert_eq!(res, 42);
}
