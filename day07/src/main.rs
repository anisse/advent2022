use itertools::Itertools;

#[derive(Debug)]
enum File {
    File { size: usize, name: String },
    Dir { name: String },
}
#[derive(Debug)]
enum Cmd {
    Cd { target: String },
    Ls { files: Vec<File> },
}

fn main() {
    let commands = parse(include_str!("../input.txt"));
    dbg!(&commands);
    //part 1
    let res = operation(&commands);
    println!("Summary: {}", res);
    //part 2
    //let res = operation2(&commands);
    //println!("Summary2: {}", res);
}
fn parse(input: &str) -> Vec<Cmd> {
    let mut cmds = Vec::new();
    let linev: Vec<&str> = input.lines().collect();
    let mut lines = linev.iter();
    while let Some(line) = lines.next() {
        let mut tokens = line.split_ascii_whitespace();
        let t = tokens.next().expect("nothing on line");
        assert_eq!(t, "$", "Unexpected token");
        let t = tokens.next().expect("no cmd");
        dbg!(&t);
        let cmd = match t {
            "cd" => Cmd::Cd {
                target: tokens.next().expect("no cd target").to_string(),
            },
            "ls" => {
                let files = lines
                    .peeking_take_while(|l| !l.starts_with('$'))
                    .map(|l| {
                        let mut ftokens = l.split_ascii_whitespace();
                        let t = ftokens.next().expect("no file element");
                        match t {
                            "dir" => File::Dir {
                                name: ftokens.next().expect("no dir name").to_string(),
                            },
                            _ => File::File {
                                size: t.parse().expect("not int"),
                                name: ftokens.next().expect("no name").to_string(),
                            },
                        }
                    })
                    .collect();
                Cmd::Ls { files }
            }
            _ => panic!("Unexpected command {}", t),
        };
        cmds.push(cmd);
    }
    cmds
}
fn operation(commands: &[Cmd]) -> usize {
    let mut count = 0;
    for _ in commands.iter() {
        if true {
            count += 1
        }
        todo!()
    }
    count
}

#[test]
fn test() {
    let commands = parse(include_str!("../sample.txt"));
    dbg!(&commands);
    //part 1
    let res = operation(&commands);
    assert_eq!(res, 42);
    //part 2
    // let res = operation2(&commands);
    // assert_eq!(res, 42);
}
