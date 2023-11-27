use advent2022::*;
fn main() {
    let (crates, instructions) = parse(input!());
    //part 1
    let res = movecrates(crates.clone(), instructions.clone());
    println!("Summary: {}", res);
    //part 2
    let res = movecrates9001(crates, instructions);
    println!("Summary 9001: {}", res);
}

#[derive(Clone, Debug)]
struct Move {
    qty: u16,
    src: usize,
    dst: usize,
}

type ParsedItem1 = Vec<char>;
type ParsedItem2 = Move;

fn parse(
    input: &str,
) -> (
    impl Iterator<Item = ParsedItem1> + Clone + '_,
    impl Iterator<Item = ParsedItem2> + Clone + '_,
) {
    let len = (input.lines().next().expect("no line").len() + 1) / 4;
    let mut inputgroups = input.split("\n\n");
    let cratesinput = inputgroups.next().expect("no crate group");
    let movesinput = inputgroups.next().expect("no moves group");
    let height = cratesinput.lines().count() - 1;
    let crateschars: Vec<Vec<char>> = cratesinput.lines().map(|l| l.chars().collect()).collect();

    let crates = (0..len).map(move |c| {
        (0..height)
            .rev()
            .map(|h| crateschars[h][1 + 4 * c])
            .filter(|el| *el != ' ')
            .collect()
    });

    let moves = movesinput.lines().map(|l| {
        let mut comp = l.split_ascii_whitespace();
        let qty = comp.nth(1).expect("No qty").parse().expect("not int");
        let src: usize = comp.nth(1).expect("No src").parse().expect("not int");
        let dst: usize = comp.nth(1).expect("No dst").parse().expect("not int");
        Move {
            qty,
            src: src - 1,
            dst: dst - 1,
        }
    });
    (crates, moves)
}
fn movecrates<I1, I2>(crates: I1, instructions: I2) -> String
where
    I1: Iterator<Item = ParsedItem1>,
    I2: Iterator<Item = ParsedItem2>,
{
    let mut crates: Vec<_> = crates.collect();
    for m in instructions {
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
fn movecrates9001<I1, I2>(crates: I1, instructions: I2) -> String
where
    I1: Iterator<Item = ParsedItem1>,
    I2: Iterator<Item = ParsedItem2>,
{
    let mut crates: Vec<_> = crates.collect();
    for m in instructions {
        let moveblock: Vec<char> = (0..m.qty)
            .map(|_| crates[m.src].pop().expect("nothing to pop !"))
            .collect();
        moveblock
            .iter()
            .rev()
            .for_each(|el| crates[m.dst].push(*el));
    }

    crates
        .iter()
        .map(|c| *c.last().expect("no last char"))
        .collect()
}

#[test]
fn test() {
    let (crates, instructions) = parse(sample!());
    //part 1
    let res = movecrates(crates.clone(), instructions.clone());
    assert_eq!(res, "CMZ");
    //part 2
    let res = movecrates9001(crates, instructions);
    assert_eq!(res, "MCD");
}
