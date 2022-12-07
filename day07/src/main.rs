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
    let res = count_small_dirs(&commands);
    dbg!(&res);
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

#[derive(Debug)]
struct DirSize {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Leaf<T> {
    children: Vec<usize>,
    parent: usize,
    el: T,
}

#[derive(Debug)]
struct Tree<T> {
    elements: Vec<Leaf<T>>,
}

/*
impl<T> Iterator for Tree<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {}
}
*/

fn eval_tree(commands: &[Cmd]) -> Tree<DirSize> {
    let mut elements: Vec<Leaf<DirSize>> = Vec::new();
    //let mut current_dir = "".to_string();
    let mut parent_id = 0;
    let mut current_id = 0;
    for cmd in commands.iter() {
        dbg!(&cmd);
        match cmd {
            Cmd::Cd { target } => {
                if target == ".." {
                    assert!(
                        parent_id != current_id,
                        "Going up too much ? {}, {}, status: {:?}",
                        current_id,
                        parent_id,
                        &elements,
                    );
                    // go up
                    (parent_id, current_id) = (elements[parent_id].parent, parent_id);
                } else {
                    //current_dir = target.clone();
                    elements.push(Leaf::<DirSize> {
                        children: Vec::new(),
                        el: DirSize {
                            name: target.clone(),
                            size: 0,
                        },
                        parent: current_id,
                    });
                    let next = elements.len() - 1;
                    if parent_id != current_id {
                        //already passed root
                        elements[parent_id].children.push(next);
                    }
                    parent_id = current_id;
                    current_id = next;
                    dbg!(parent_id);
                    dbg!(current_id);
                    dbg!(&target);
                }
            }
            Cmd::Ls { files } => {
                files.iter().for_each(|f| match f {
                    File::File { size, .. } => {
                        let mut x = current_id;
                        loop {
                            dbg!(size);
                            dbg!(&elements[x].el.name);
                            elements[x].el.size += size;
                            if x == 0 {
                                break;
                            }
                            x = elements[x].parent;
                        }
                    }
                    File::Dir { .. } => {}
                });
            }
        }
    }
    Tree { elements }
}
fn count_small_dirs(commands: &[Cmd]) -> usize {
    let tree = eval_tree(commands);
    dbg!(&tree);
    tree.elements
        .iter()
        .map(|ds| ds.el.size)
        .filter(|s| *s < 100000)
        .sum()
}

#[test]
fn test() {
    let commands = parse(include_str!("../sample.txt"));
    dbg!(&commands);
    //part 1
    let res = count_small_dirs(&commands);
    assert_eq!(res, 95437);
    //part 2
    // let res = operation2(&commands);
    // assert_eq!(res, 42);
}
